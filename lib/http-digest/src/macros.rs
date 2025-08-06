macro_rules! define_digest_algorithm {
    ($digest_algorithm:ident, $digest_size:literal) => {
        #[derive(::core::fmt::Debug)]
        pub struct $digest_algorithm([u8; $digest_size]);

        impl ::core::convert::TryFrom<::std::vec::Vec<u8>> for $digest_algorithm {
            type Error = $crate::ValueParseError;

            fn try_from(value: ::std::vec::Vec<u8>) -> ::core::result::Result<Self, Self::Error> {
                ::core::result::Result::Ok(Self(value.try_into()?))
            }
        }

        impl ::std::str::FromStr for $digest_algorithm {
            type Err = $crate::ValueParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(
                    ::base64::prelude::BASE64_STANDARD.decode(
                        s.strip_prefix(":")
                            .unwrap_or(s)
                            .strip_suffix(":")
                            .unwrap_or(s),
                    )?,
                )
            }
        }
    };
}

pub(super) use define_digest_algorithm;
