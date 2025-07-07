use std::str;

use serde::de::Unexpected;
use serde::ser;
use serde::ser::Impossible;
use serde::ser::Serialize;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;

pub struct Entity<S>(pub S);

pub trait EntitySerializer: Sized {
    type Ok;
    type Error: ser::Error;
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error>;

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error>;

    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error>;

    fn serialize_seq(self) -> Result<Self::SerializeSeq, Self::Error>;

    fn unsupported<T>(self, unexp: Unexpected) -> Result<T, Self::Error>;
}

impl<S> Serializer for Entity<S>
where
    S: EntitySerializer,
{
    type Ok = S::Ok;
    type Error = S::Error;
    type SerializeSeq = S::SerializeSeq;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_float(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_float(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(v.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_bytes(value)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_none()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_some(value)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.0.unsupported(Unexpected::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(variant)
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
        self.0.unsupported(Unexpected::NewtypeVariant)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.serialize_seq()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.unsupported(Unexpected::Other("tuple"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.unsupported(Unexpected::Other("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0.unsupported(Unexpected::TupleVariant)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.unsupported(Unexpected::Map)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.unsupported(Unexpected::Other("struct"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0.unsupported(Unexpected::StructVariant)
    }
}

impl<S> Entity<S>
where
    S: EntitySerializer,
{
    #[cfg(feature = "itoa")]
    fn serialize_integer<I>(self, value: I) -> Result<S::Ok, S::Error>
    where
        I: itoa::Integer,
    {
        let mut buf = itoa::Buffer::new();
        self.serialize_str(buf.format(value))
    }

    #[cfg(not(feature = "itoa"))]
    fn serialize_integer<I>(self, value: I) -> Result<S::Ok, S::Error>
    where
        I: ToString,
    {
        let value_str = value.to_string();
        self.0.serialize_str(&value_str)
    }

    #[cfg(feature = "ryu")]
    fn serialize_float<F>(self, value: F) -> Result<S::Ok, S::Error>
    where
        F: ryu::Float,
    {
        let mut buf = ryu::Buffer::new();
        self.serialize_str(buf.format(value))
    }

    #[cfg(not(feature = "ryu"))]
    fn serialize_float<F>(self, value: F) -> Result<S::Ok, S::Error>
    where
        F: ToString,
    {
        let value_str = value.to_string();
        self.0.serialize_str(&value_str)
    }
}
