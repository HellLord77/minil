use std::mem;

use ser::Impossible;
use ser::Serialize;
use ser::SerializeTuple;
use serde::Serializer;
use serde::de::Unexpected;
use serde::ser;

use crate::ser::entity::Entity;
use crate::ser::error::Error;
use crate::ser::name::NameEntity;
use crate::ser::value::ValueEntity;
use crate::types::HeaderNameOwned;
use crate::types::HeaderOwnedSeqRef;

#[derive(Debug)]
enum HeaderState {
    SerializingName,
    SerializingValue(HeaderNameOwned),
    Serialized,
}

#[derive(Debug)]
pub struct Header<'ser> {
    headers: HeaderOwnedSeqRef<'ser>,
    state: HeaderState,
}

impl<'ser> Header<'ser> {
    #[inline]
    pub fn new(headers: HeaderOwnedSeqRef<'ser>) -> Self {
        Self {
            headers,
            state: HeaderState::SerializingName,
        }
    }
}

impl<'ser> Serializer for Header<'ser> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Signed(v as i64))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Signed(v as i64))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Signed(v as i64))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Signed(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Unsigned(v as u64))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Unsigned(v as u64))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Unsigned(v as u64))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Unsigned(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Float(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Str(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Bytes(v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Unit)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::Other("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Error::unsupported_header(Unexpected::UnitVariant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Error::unsupported_header(Unexpected::NewtypeVariant)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Error::unsupported_header(Unexpected::Seq)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if len == 2 {
            Ok(self)
        } else {
            Error::unsupported_header(Unexpected::Other("tuple"))
        }
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Error::unsupported_header(Unexpected::Other("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Error::unsupported_header(Unexpected::TupleVariant)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Error::unsupported_header(Unexpected::Map)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Error::unsupported_header(Unexpected::Other("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Error::unsupported_header(Unexpected::StructVariant)
    }
}

impl<'ser> SerializeTuple for Header<'ser> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match mem::replace(&mut self.state, HeaderState::Serialized) {
            HeaderState::SerializingName => {
                let name = value.serialize(Entity(NameEntity::new(|name| Ok(name.to_owned()))))?;
                self.state = HeaderState::SerializingValue(name);
                Ok(())
            }
            HeaderState::SerializingValue(name) => {
                match value.serialize(Entity(ValueEntity::new(self.headers, &name))) {
                    Ok(_) => {
                        self.state = HeaderState::Serialized;
                        Ok(())
                    }
                    Err(err) => {
                        self.state = HeaderState::SerializingValue(name);
                        Err(err)
                    }
                }
            }
            HeaderState::Serialized => Err(Error::header_done()),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.state {
            HeaderState::Serialized => Ok(self.headers),
            _ => Err(Error::header_not_done()),
        }
    }
}
