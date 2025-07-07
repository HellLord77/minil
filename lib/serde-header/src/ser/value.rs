use serde::Serialize;
use serde::de::Unexpected;
use serde::ser::SerializeSeq;

use crate::ser::entity::Entity;
use crate::ser::entity::EntitySerializer;
use crate::ser::error::Error;
use crate::types::HeaderNameRef;
use crate::types::HeaderOwnedSeqRef;

#[derive(Debug)]
pub struct ValueEntity<'ser, 'name> {
    headers: HeaderOwnedSeqRef<'ser>,
    name: HeaderNameRef<'name>,
    nested: bool,
}

impl<'ser, 'name> ValueEntity<'ser, 'name> {
    #[inline]
    pub fn new(headers: HeaderOwnedSeqRef<'ser>, name: HeaderNameRef<'name>) -> Self {
        Self {
            headers,
            name,
            nested: false,
        }
    }

    #[inline]
    pub fn new_nested(headers: HeaderOwnedSeqRef<'ser>, name: HeaderNameRef<'name>) -> Self {
        Self {
            headers,
            name,
            nested: true,
        }
    }
}

impl<'ser, 'name> EntitySerializer for ValueEntity<'ser, 'name> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;
    type SerializeSeq = Self;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(value.as_bytes())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
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

impl<'ser, 'name> SerializeSeq for ValueEntity<'ser, 'name> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let nested = ValueEntity::new_nested(self.headers, self.name);
        value.serialize(Entity(nested))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }
}
