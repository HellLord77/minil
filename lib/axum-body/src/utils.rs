use axum_core::body::Body;
use axum_core::extract::FromRequest;
use axum_core::extract::Request;
use axum_core::extract::rejection::BytesRejection;
use bytes::Bytes;
use tokio_stream::StreamExt;
use tokio_stream::once;

pub(super) async fn cloned2<S>(
    req: Request,
    state: &S,
) -> Result<(Request, Request), BytesRejection>
where
    S: Send + Sync,
{
    let (parts, body) = req.into_parts();
    let (parts1, parts2) = (parts.clone(), parts.clone());
    let req = Request::from_parts(parts, body);

    let bytes = Bytes::from_request(req, state).await?;
    let req1 = Request::from_parts(parts1, Body::from(bytes.clone()));
    let req2 = Request::from_parts(parts2, Body::from(bytes));

    Ok((req1, req2))
}

pub(super) async fn has_remaining(body: Body) -> Result<(bool, Body), axum_core::Error> {
    let mut stream = body.into_data_stream();

    if let Some(chunk) = stream.try_next().await? {
        let stream = once(Ok(chunk)).chain(stream);
        let body = Body::from_stream(stream);

        Ok((true, body))
    } else {
        Ok((false, Body::empty()))
    }
}
