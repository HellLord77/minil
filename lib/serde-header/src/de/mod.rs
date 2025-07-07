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

pub fn from_header_seq<'de, T>(input: &'de HeaderRefSeq<'de>) -> Result<T, Error>
where
    T: serde::Deserialize<'de>,
{
    T::deserialize(Deserializer::new(input))
}

#[cfg(feature = "httparse")]
pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T, Error>
where
    T: serde::Deserialize<'de>,
{
    let mut headers = vec![httparse::EMPTY_HEADER; 30];

    loop {
        match httparse::parse_headers(input, &mut headers) {
            Ok(httparse::Status::Complete((_, headers))) => {
                break T::deserialize(Deserializer::from_headers(headers));
            }
            Ok(httparse::Status::Partial) => {
                break Err(de::Error::custom(
                    "could not parse headers: incomplete headers",
                ));
            }
            Err(httparse::Error::TooManyHeaders) => {
                headers.resize_with(headers.len() * 2, || httparse::EMPTY_HEADER);
            }
            Err(err) => {
                break Err(de::Error::custom(format!("could not parse headers: {err}")));
            }
        }
    }
}

#[cfg(feature = "httparse")]
pub fn from_str<'de, T>(input: &'de str) -> Result<T, Error>
where
    T: serde::Deserialize<'de>,
{
    from_bytes(input.as_bytes())
}

#[cfg(feature = "httparse")]
pub fn from_reader<T, R>(mut input: R) -> Result<T, Error>
where
    T: de::DeserializeOwned,
    R: std::io::Read,
{
    let mut buf = vec![];
    input
        .read_to_end(&mut buf)
        .map_err(|err| de::Error::custom(format!("could not read input: {err}")))?;
    from_bytes(&buf)
}

#[cfg(feature = "http")]
pub fn from_header_map<'de, I, T>(input: I) -> Result<T, Error>
where
    I: IntoIterator<Item = (&'de http::HeaderName, &'de http::HeaderValue)>,
    T: serde::Deserialize<'de>,
{
    T::deserialize(Deserializer::from_header_map(input))
}

pub struct Deserializer<'de>(Box<HeaderRefSeq<'de>>);

impl<'de> Deserializer<'de> {
    #[inline]
    pub fn new(headers: &'de HeaderRefSeq<'de>) -> Self {
        Self(headers.into())
    }

    #[cfg(feature = "httparse")]
    #[inline]
    pub fn from_headers<'item, I>(headers: I) -> Self
    where
        'de: 'item,
        I: IntoIterator<Item = &'item httparse::Header<'de>>,
    {
        Self(headers.into_iter().map(|h| (h.name, h.value)).collect())
    }

    #[cfg(feature = "http")]
    #[inline]
    pub fn from_header_map<I>(headers: I) -> Self
    where
        I: IntoIterator<Item = (&'de http::HeaderName, &'de http::HeaderValue)>,
    {
        Self(
            headers
                .into_iter()
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
            return Err(de::Error::custom(format!(
                "duplicate caseless field {}",
                fields.join(", ")
            )));
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
