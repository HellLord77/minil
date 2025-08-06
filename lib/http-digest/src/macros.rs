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
    };
}

pub(super) use define_digest_algorithm;
