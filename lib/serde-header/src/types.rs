pub type HeaderName = str;
pub type HeaderValue = [u8];

pub type HeaderNameRef<'h> = &'h HeaderName;
pub type HeaderValueRef<'h> = &'h HeaderValue;

pub type Header = (HeaderName, HeaderValue);
pub type HeaderSeq = [Header];

pub type HeaderRef<'h> = (HeaderNameRef<'h>, HeaderValueRef<'h>);
pub type HeaderRefSeq<'h> = [HeaderRef<'h>];
