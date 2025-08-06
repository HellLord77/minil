use base64::DecodeError;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use strum::ParseError;

#[derive(Debug, Display, From, Error)]
pub enum ValueParseError {
    #[from(skip)]
    PrefixColonNotFound(#[error(not(source))] String),

    #[from(skip)]
    SuffixColonNotFound(#[error(not(source))] String),

    InvalidBase64(DecodeError),

    #[display("InvalidLength")]
    InvalidLength(#[error(not(source))] Vec<u8>),
}

#[derive(Debug, Display, From, Error)]
pub enum DigestParseError {
    SeparatorCommaNotFound(#[error(not(source))] String),

    InvalidAlgorithm(ParseError),

    InvalidValue(ValueParseError),
}
