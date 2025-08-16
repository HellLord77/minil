use crc_fast::Digest;
use digest::DynDigest;

pub(crate) trait DigestExt {
    fn finalize_vec(self) -> Vec<u8>;
}

impl DigestExt for Digest {
    fn finalize_vec(self) -> Vec<u8> {
        let result = self.finalize();

        if self.output_size() == 4 {
            result.to_be_bytes()[4..].to_vec()
        } else {
            result.to_be_bytes().to_vec()
        }
    }
}
