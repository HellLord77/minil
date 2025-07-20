use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use url::ParseError;
use url::Url;

pub type UrlParseResult = Result<Url, UrlParseError>;

#[derive(Debug, Display, From, Error)]
pub enum UrlParseError {
    Scheme,
    Host(ParseError),
    Port,
    Username,
    Password,
}
