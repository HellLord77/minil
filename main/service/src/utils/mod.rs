use mime::Mime;

mod chunked_decoder;

pub(super) use chunked_decoder::ChunkedDecoder;

pub(super) fn get_mime(path: &str, bytes: &[u8]) -> Mime {
    let mimes_ext = mime_guess::from_path(path);
    let mime_sig = infer::get(bytes).map(|mime| {
        mime.mime_type()
            .parse::<Mime>()
            .unwrap_or_else(|_err| unreachable!())
    });

    mime_sig.unwrap_or_else(|| mimes_ext.first_or_octet_stream())
}

#[cfg(test)]
mod tests {
    use mime::APPLICATION_OCTET_STREAM;
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
        assert_eq!(mime, TEXT_PLAIN);
    }

    #[test]
    fn test_get_mime_path() {
        let path = "example.csv";
        let bytes = b"Hello, world!";

        let mime = get_mime(path, bytes);
        assert_eq!(mime, TEXT_CSV);
    }

    #[test]
    fn test_get_mime_bytes() {
        let path = "example";
        let bytes = &[0xFF, 0xD8, 0xFF, 0xAA];

        let mime = get_mime(path, bytes);
        assert_eq!(mime, IMAGE_JPEG);
    }

    #[test]
    fn test_get_mime_unknown() {
        let path = "example";
        let bytes = b"Hello, world!";

        let mime = get_mime(path, bytes);
        assert_eq!(mime, APPLICATION_OCTET_STREAM);
    }

    #[test]
    fn test_get_mime_dubious_path() {
        let path = "example.gif";
        let bytes = &[0xFF, 0xD8, 0xFF, 0xAA];

        let mime = get_mime(path, bytes);
        assert_eq!(mime, IMAGE_JPEG);
    }

    #[test]
    fn test_get_mime_dubious_bytes() {
        let path = "example.xml";
        let bytes = b"{\"key\": \"value\"}";

        let mime = get_mime(path, bytes);
        assert_eq!(mime, TEXT_XML);
    }
}
