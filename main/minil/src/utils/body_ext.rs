use std::io;

use axum::body::Body;
use futures::StreamExt;
use tokio::io::AsyncRead;
use tokio_util::io::StreamReader;

pub(crate) trait BodyExt {
    fn into_data_read(self) -> impl AsyncRead;
}

impl BodyExt for Body {
    fn into_data_read(self) -> impl AsyncRead {
        StreamReader::new(
            self.into_data_stream()
                .map(|res| res.map_err(|err| io::Error::other(err.into_inner()))),
        )
    }
}
