pub mod entity;
pub mod error;
pub mod header;
pub mod name;
pub mod value;

use serde::Serialize;
use serde::de::Unexpected;
use serde::ser;
use serde::ser::Impossible;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::ser::SerializeTuple;

use crate::ser::entity::Entity;
use crate::ser::error::Error;
use crate::ser::header::Header;
use crate::ser::name::NameEntity;
use crate::ser::value::ValueEntity;
use crate::types::HeaderNameOwned;
use crate::types::HeaderOwnedSeq;
use crate::types::HeaderOwnedSeqRef;

pub fn to_header_seq<T>(input: T) -> Result<HeaderOwnedSeq, Error>
where
    T: Serialize,
{
    let mut headers = vec![];
    input.serialize(Serializer::new(&mut headers))?;
    Ok(headers)
}

pub fn to_bytes<T>(input: T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    let mut bytes = vec![];

    for (name, value) in to_header_seq(input)? {
        bytes.extend(name.as_bytes());
        bytes.extend(b": ");
        bytes.extend(value);
        bytes.extend(b"\r\n");
    }
    bytes.extend(b"\r\n");

    Ok(bytes)
}

pub fn to_string<T>(input: T) -> Result<String, Error>
where
    T: Serialize,
{
    String::from_utf8(to_bytes(input)?)
        .map_err(|err| format!("could not convert to string: {err}").into())
}

pub fn to_writer<W, T>(mut writer: W, input: &T) -> Result<W, Error>
where
    W: std::io::Write,
    T: Serialize,
{
    let headers = to_bytes(input)?;
    writer
        .write_all(&headers)
        .map_err(|err| format!("could not write headers: {err}"))?;
    Ok(writer)
}

#[cfg(feature = "http")]
pub fn to_header_map<T>(input: T) -> Result<http::HeaderMap, Error>
where
    T: Serialize,
{
    let headers = to_header_seq(input)?;
    let mut map = http::HeaderMap::with_capacity(headers.len());

    for (name, value) in headers {
        let name = name
            .parse::<http::HeaderName>()
            .map_err(|err| format!("could not convert to header name: {err}"))?;
        let value = value
            .try_into()
            .map_err(|err| format!("could not convert to header value: {err}"))?;
        map.append(name, value);
    }

    Ok(map)
}

#[derive(Debug)]
pub struct Serializer<'ser> {
    headers: HeaderOwnedSeqRef<'ser>,
}

impl<'ser> Serializer<'ser> {
    #[inline]
    pub fn new(headers: HeaderOwnedSeqRef<'ser>) -> Self {
        Self { headers }
    }
}

impl<'ser> ser::Serializer for Serializer<'ser> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = MapSerializer<'ser>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Signed(v as i64))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Signed(v as i64))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Signed(v as i64))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Signed(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Unsigned(v as u64))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Unsigned(v as u64))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Unsigned(v as u64))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Unsigned(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Float(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Float(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Char(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Str(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::Bytes(v))
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
        Ok(self.headers)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Error::unsupported(Unexpected::UnitVariant)
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
        Error::unsupported(Unexpected::NewtypeVariant)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Error::unsupported(Unexpected::Other("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Error::unsupported(Unexpected::TupleVariant)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self.headers))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Error::unsupported(Unexpected::StructVariant)
    }
}

impl<'ser> SerializeSeq for Serializer<'ser> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(Header::new(self.headers))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }
}

impl<'ser> SerializeTuple for Serializer<'ser> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(Header::new(self.headers))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }
}

impl<'ser> SerializeStruct for Serializer<'ser> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(Entity(ValueEntity::new(self.headers, key)))?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.headers)
    }
}

#[derive(Debug)]
pub struct MapSerializer<'ser> {
    headers: HeaderOwnedSeqRef<'ser>,
    name: Option<HeaderNameOwned>,
}

impl<'ser> MapSerializer<'ser> {
    #[inline]
    pub fn new(headers: HeaderOwnedSeqRef<'ser>) -> Self {
        Self {
            headers,
            name: None,
        }
    }
}

impl<'ser> SerializeMap for MapSerializer<'ser> {
    type Ok = HeaderOwnedSeqRef<'ser>;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let name = key.serialize(Entity(NameEntity::new(|name| Ok(name.to_owned()))))?;
        self.name = Some(name);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let name = self.name.as_ref().ok_or_else(Error::map_no_name)?;
        value.serialize(Entity(ValueEntity::new(self.headers, name)))?;
        self.name = None;
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let name = key.serialize(Entity(NameEntity::new(|name| Ok(name.to_owned()))))?;
        value.serialize(Entity(ValueEntity::new(self.headers, &name)))?;
        self.name = None;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.name {
            Some(_) => Err(Error::map_no_value()),
            None => Ok(self.headers),
        }
    }
}
