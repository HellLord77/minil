pub type HeaderNameOwned = String;
pub type HeaderValueOwned = Vec<u8>;

pub type HeaderOwned = (HeaderNameOwned, HeaderValueOwned);
pub type HeaderOwnedSeq = Vec<HeaderOwned>;

pub type HeaderOwnedSeqRef<'h> = &'h mut HeaderOwnedSeq;

pub type HeaderName = str;
pub type HeaderValue = [u8];

pub type HeaderNameRef<'h> = &'h HeaderName;
pub type HeaderValueRef<'h> = &'h HeaderValue;

pub type Header = (HeaderName, HeaderValue);
pub type HeaderSeq = [Header];

pub type HeaderRef<'h> = (HeaderNameRef<'h>, HeaderValueRef<'h>);
pub type HeaderRefSeq<'h> = [HeaderRef<'h>];
