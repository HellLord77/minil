use crate::{
    category::Category,
    rejection::{MissingXmlContentType, XmlDataError, XmlRejection, XmlSyntaxError},
};
use axum_core::{
    extract::{FromRequest, OptionalFromRequest, Request},
    response::{IntoResponse, Response},
};
use bytes::{BufMut, Bytes, BytesMut};
use http::{
    StatusCode,
    header::{self, HeaderMap, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Xml<T>(pub T);

impl<T, S> FromRequest<S> for Xml<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = XmlRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if !xml_content_type(req.headers()) {
            return Err(MissingXmlContentType.into());
        }

        let bytes = Bytes::from_request(req, state).await?;
        Self::from_bytes(&bytes)
    }
}

impl<T, S> OptionalFromRequest<S> for Xml<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = XmlRejection;

    async fn from_request(req: Request, state: &S) -> Result<Option<Self>, Self::Rejection> {
        let headers = req.headers();
        if headers.get(header::CONTENT_TYPE).is_some() {
            if xml_content_type(headers) {
                let bytes = Bytes::from_request(req, state).await?;
                Ok(Some(Self::from_bytes(&bytes)?))
            } else {
                Err(MissingXmlContentType.into())
            }
        } else {
            Ok(None)
        }
    }
}

fn xml_content_type(headers: &HeaderMap) -> bool {
    let Some(content_type) = headers.get(header::CONTENT_TYPE) else {
        return false;
    };

    let Ok(content_type) = content_type.to_str() else {
        return false;
    };

    let Ok(mime) = content_type.parse::<mime::Mime>() else {
        return false;
    };

    let is_xml_content_type = mime.type_() == "application"
        && (mime.subtype() == "xml" || mime.suffix().is_some_and(|name| name == "xml"));

    is_xml_content_type
}

axum_core::__impl_deref!(Xml);

impl<T> From<T> for Xml<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Xml<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, XmlRejection> {
        fn make_rejection(err: serde_path_to_error::Error<quick_xml::DeError>) -> XmlRejection {
            match Category::classify(err.inner()) {
                Category::Data => XmlDataError::from_err(err).into(),
                Category::Syntax | Category::Eof => XmlSyntaxError::from_err(err).into(),
                Category::Io => {
                    if cfg!(debug_assertions) {
                        unreachable!()
                    } else {
                        XmlSyntaxError::from_err(err).into()
                    }
                }
            }
        }

        let deserializer = &mut quick_xml::de::Deserializer::from_reader(bytes);

        match serde_path_to_error::deserialize(deserializer) {
            Ok(value) => Ok(Xml(value)),
            Err(err) => Err(make_rejection(err)),
        }
    }
}

impl<T> IntoResponse for Xml<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        fn make_response(
            buf: BytesMut,
            ser_result: Result<quick_xml::se::WriteResult, quick_xml::SeError>,
        ) -> Response {
            match ser_result {
                Ok(_) => (
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/xml"),
                    )],
                    buf.freeze(),
                )
                    .into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )],
                    err.to_string(),
                )
                    .into_response(),
            }
        }

        let mut buf = BytesMut::with_capacity(128).writer();
        let res = quick_xml::se::to_utf8_io_writer(&mut buf, &self.0);
        make_response(buf.into_inner(), res)
    }
}
