use mime::Mime;

mod chunk_decoder;

pub(super) use chunk_decoder::ChunkDecoder;

pub(super) fn get_mime(path: &str, bytes: &[u8]) -> Option<Mime> {
    infer::get(bytes)
        .map(|mime| mime.mime_type().parse::<Mime>().unwrap())
        .or_else(|| mime_guess::from_path(path).first())
}

#[cfg(test)]
mod tests {
    use mime::IMAGE_JPEG;
    use mime::TEXT_CSV;
    use mime::TEXT_PLAIN;
    use mime::TEXT_XML;

    use super::*;

    #[test]
    fn test_get_mime() {
        let path = "example.txt";
        let bytes = b"Hello, world!";

        let mime = get_mime(path, bytes);
        assert_eq!(mime, Some(TEXT_PLAIN));
    }

    #[test]
    fn test_get_mime_path() {
        let path = "example.csv";
        let bytes = b"Hello, world!";

        let mime = get_mime(path, bytes);
        assert_eq!(mime, Some(TEXT_CSV));
    }

    #[test]
    fn test_get_mime_bytes() {
        let path = "example";
        let bytes = &[0xFF, 0xD8, 0xFF, 0xAA];

        let mime = get_mime(path, bytes);
        assert_eq!(mime, Some(IMAGE_JPEG));
    }

    #[test]
    fn test_get_mime_unknown() {
        let path = "example";
        let bytes = b"Hello, world!";

        let mime = get_mime(path, bytes);
        assert_eq!(mime, None);
    }

    #[test]
    fn test_get_mime_dubious_path() {
        let path = "example.gif";
        let bytes = &[0xFF, 0xD8, 0xFF, 0xAA];

        let mime = get_mime(path, bytes);
        assert_eq!(mime, Some(IMAGE_JPEG));
    }

    #[test]
    fn test_get_mime_dubious_bytes() {
        let path = "example.xml";
        let bytes = b"{\"key\": \"value\"}";

        let mime = get_mime(path, bytes);
        assert_eq!(mime, Some(TEXT_XML));
    }
}
