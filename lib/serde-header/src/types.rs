pub type HeaderName = str;
pub type HeaderValue = [u8];

pub type HeaderNameRef<'a> = &'a HeaderName;
pub type HeaderValueRef<'a> = &'a HeaderValue;

pub type Header = (HeaderName, HeaderValue);
pub type HeaderSeq = [Header];

pub type HeaderRef<'a> = (HeaderNameRef<'a>, HeaderValueRef<'a>);
pub type HeaderRefSeq<'a> = [HeaderRef<'a>];
