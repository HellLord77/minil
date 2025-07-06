use ser::Impossible;
use serde::Serialize;
use serde::de::Unexpected;
use serde::ser;

use crate::ser::entity::EntitySerializer;
use crate::ser::error::Error;

pub struct NameEntity<End>(End);

impl<End, Ok> NameEntity<End>
where
    End: FnOnce(&str) -> Result<Ok, Error>,
{
    #[inline]
    pub fn new(end: End) -> Self {
        Self(end)
    }
}

impl<End, Ok> EntitySerializer for NameEntity<End>
where
    End: FnOnce(&str) -> Result<Ok, Error>,
{
    type Ok = Ok;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.0(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(str::from_utf8(value)?)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.unsupported(Unexpected::Option)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.unsupported(Unexpected::Option)
    }

    fn serialize_seq(self) -> Result<Self::SerializeSeq, Self::Error> {
        self.unsupported(Unexpected::Seq)
    }

    fn unsupported<T>(self, unexp: Unexpected) -> Result<T, Self::Error> {
        Error::unsupported_name(unexp)
    }
}
