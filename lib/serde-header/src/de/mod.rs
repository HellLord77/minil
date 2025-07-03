pub mod name;
pub mod unit_only;
pub mod val_or_vec;
pub mod value;

#[cfg(feature = "caseless")]
pub mod caseless_map;

use indexmap::IndexMap;
use indexmap::map::Entry;
use serde::de;
use serde::de::IntoDeserializer;
use serde::de::Visitor;
use serde::de::value::Error;
use serde::de::value::MapDeserializer;
use serde::forward_to_deserialize_any;

use crate::de::name::Name;
use crate::de::val_or_vec::ValOrVec;
use crate::de::value::Value;
use crate::types::HeaderRefSeq;

pub fn from_headers<'de, T>(input: &'de HeaderRefSeq<'de>) -> Result<T, Error>
where
    T: serde::Deserialize<'de>,
{
    T::deserialize(Deserializer::from_headers(input))
}

#[cfg(feature = "httparse")]
pub fn try_from_bytes<'de, T>(input: &'de [u8]) -> Result<Result<T, Error>, httparse::Error>
where
    T: serde::Deserialize<'de>,
{
    Ok(T::deserialize(Deserializer::try_from_bytes(input)?))
}

#[cfg(feature = "http")]
pub fn from_header_map<'de, T>(input: &'de http::HeaderMap) -> Result<T, Error>
where
    T: serde::Deserialize<'de>,
{
    T::deserialize(Deserializer::from_header_map(input))
}

pub struct Deserializer<'de>(Box<HeaderRefSeq<'de>>);

impl<'de> Deserializer<'de> {
    #[inline]
    pub fn from_headers(headers: &'de HeaderRefSeq<'de>) -> Self {
        Self(headers.into())
    }

    #[cfg(feature = "httparse")]
    #[inline]
    pub fn try_from_bytes(bytes: &'de [u8]) -> Result<Self, httparse::Error> {
        let mut headers = vec![httparse::EMPTY_HEADER; 30];

        loop {
            match httparse::parse_headers(bytes, &mut headers) {
                Ok(httparse::Status::Complete((_, headers))) => {
                    return Ok(Self(headers.iter().map(|h| (h.name, h.value)).collect()));
                }
                Ok(httparse::Status::Partial) => return Err(httparse::Error::TooManyHeaders),
                Err(httparse::Error::TooManyHeaders) => {
                    headers.resize_with(headers.len() * 2, || httparse::EMPTY_HEADER);
                }
                Err(err) => return Err(err),
            }
        }
    }

    #[cfg(feature = "http")]
    #[inline]
    pub fn from_header_map(headers: &'de http::HeaderMap) -> Self {
        Self(
            headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_bytes()))
                .collect(),
        )
    }
}

impl<'de> IntoDeserializer<'de> for Deserializer<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let deserializer = MapDeserializer::new(header_iterator(&self.0));
        deserializer.end()?;
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(MapDeserializer::new(header_iterator(&self.0)))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(MapDeserializer::new(group_entries(&self.0).into_iter()))
    }

    #[cfg(feature = "caseless")]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let canonical_fields: indexmap::IndexSet<_> =
            indexmap::IndexSet::from_iter(fields.iter().copied().map(Name));
        if fields.len() > canonical_fields.len() {
            return Err(de::Error::custom("expected field with unique name"));
        }

        let map = MapDeserializer::new(group_entries(&self.0).into_iter());
        let field_map = std::iter::zip(canonical_fields, fields.iter().copied());
        visitor.visit_map(caseless_map::CaselessMap::new(map, field_map))
    }

    #[cfg(not(feature = "caseless"))]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    forward_to_deserialize_any! {
        bool
        i8
        i16
        i32
        i64
        i128
        u8
        u16
        u32
        u64
        u128
        f32
        f64
        char
        str
        string
        bytes
        byte_buf
        option
        unit_struct
        tuple
        tuple_struct
        enum
        identifier
        ignored_any
    }
}

fn header_iterator<'de>(
    headers: &HeaderRefSeq<'de>,
) -> impl Iterator<Item = (Name<'de>, Value<'de>)> {
    headers
        .iter()
        .map(|(name, value)| (Name(name), Value(value)))
}

fn group_entries<'de>(headers: &HeaderRefSeq<'de>) -> IndexMap<Name<'de>, ValOrVec<'de>> {
    let mut map = IndexMap::new();

    for (name, value) in headers {
        match map.entry(Name(name)) {
            Entry::Vacant(v) => {
                v.insert(ValOrVec::Val(Value(value)));
            }
            Entry::Occupied(mut o) => {
                o.get_mut().push(Value(value));
            }
        }
    }

    map
}
