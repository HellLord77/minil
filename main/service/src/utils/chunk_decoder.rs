use std::io;

use bytes::Bytes;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::Decoder;

pub(crate) struct ChunkDecoder {
    capacity: usize,
}

impl ChunkDecoder {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self::with_capacity(0)
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self { capacity }
    }
}

impl Decoder for ChunkDecoder {
    type Item = Bytes;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() >= self.capacity {
            Ok(Some(src.split_to(self.capacity).freeze()))
        } else {
            Ok(None)
        }
    }

    fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            Ok(None)
        } else {
            Ok(Some(src.split().freeze()))
        }
    }
}
