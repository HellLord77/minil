use serde::Serialize;
use serde::de::Unexpected;
use serde::ser::SerializeSeq;

use crate::ser::entity::Entity;
use crate::ser::entity::EntitySerializer;
use crate::ser::error::Error;
use crate::types::HeaderNameRef;
use crate::types::HeaderOwnedSeq;

#[derive(Debug, Default)]
pub struct ValueEntity<'ser> {
    headers: HeaderOwnedSeq,
    name: HeaderNameRef<'ser>,
    nested: bool,
}

impl<'ser> ValueEntity<'ser> {
    #[inline]
    pub fn new(name: HeaderNameRef<'ser>) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }

    #[inline]
    pub fn new_nested(name: HeaderNameRef<'ser>) -> Self {
        Self {
            name,
            nested: true,
            ..Self::default()
        }
    }
}

impl<'ser> EntitySerializer for ValueEntity<'ser> {
    type Ok = HeaderOwnedSeq;
    type Error = Error;
    type SerializeSeq = Self;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(value.as_bytes())
    }

    fn serialize_bytes(mut self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.headers.push((self.name.to_owned(), value.to_owned()));
        Ok(self.headers)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(Entity(self))
    }

    fn serialize_seq(self) -> Result<Self::SerializeSeq, Self::Error> {
        if self.nested {
            self.unsupported(Unexpected::Seq)
        } else {
            Ok(self)
        }
    }

    fn unsupported<T>(self, unexp: Unexpected) -> Result<T, Self::Error> {
        Error::unsupported_value(unexp)
    }
}

impl<'ser> SerializeSeq for ValueEntity<'ser> {
    type Ok = HeaderOwnedSeq;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let headers = value.serialize(Entity(Self::new_nested(self.name)))?;
        self.headers.extend(headers);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }
}
