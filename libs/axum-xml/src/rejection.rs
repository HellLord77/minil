use axum_core::__composite_rejection as composite_rejection;
use axum_core::__define_rejection as define_rejection;
use axum_core::extract::rejection::BytesRejection;

define_rejection! {
    #[status = UNPROCESSABLE_ENTITY]
    #[body = "Failed to deserialize the XML body into the target type"]
    pub struct XmlDataError(Error);
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "Failed to parse the request body as XML"]
    pub struct XmlSyntaxError(Error);
}

define_rejection! {
    #[status = UNSUPPORTED_MEDIA_TYPE]
    #[body = "Expected request with `Content-Type: application/xml`"]
    pub struct MissingXmlContentType;
}

composite_rejection! {
    pub enum XmlRejection {
        XmlDataError,
        XmlSyntaxError,
        MissingXmlContentType,
        BytesRejection,
    }
}
