macro_rules! define_digest_algorithm {
    ($algorithm:ident, $size:literal) => {::paste::paste! {
        pub type [<$algorithm Bytes>] = [u8; $size];

        #[derive(::core::fmt::Debug, ::core::hash::Hash, ::derive_more::From, ::derive_more::AsRef)]
        pub struct [<Digest $algorithm>]([<$algorithm Bytes>]);

        impl [<Digest $algorithm>] {
            #[inline]
            #[must_use]
            pub fn as_bytes(&self) -> &[<$algorithm Bytes>] {
                &self.0
            }

            #[inline]
            #[must_use]
            pub fn into_bytes(self) -> [<$algorithm Bytes>] {
                self.0
            }
        }

        impl ::core::convert::From<[<Digest $algorithm>]> for ::std::vec::Vec<u8> {
            fn from(value: [<Digest $algorithm>]) -> Self {
                value.0.to_vec()
            }
        }

        impl ::core::convert::TryFrom<::std::vec::Vec<u8>> for [<Digest $algorithm>] {
            type Error = $crate::ValueParseError;

            fn try_from(value: ::std::vec::Vec<u8>) -> ::core::result::Result<Self, Self::Error> {
                ::core::result::Result::Ok(Self(value.try_into()?))
            }
        }

        impl ::std::str::FromStr for [<Digest $algorithm>] {
            type Err = $crate::ValueParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(
                    ::base64::prelude::BASE64_STANDARD.decode(
                        s.strip_prefix(':')
                            .unwrap_or(s)
                            .strip_suffix(':')
                            .unwrap_or(s),
                    )?,
                )
            }
        }
    }};
}

pub(super) use define_digest_algorithm;
