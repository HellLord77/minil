use std::iter;

use de::DeserializeSeed;
use de::Deserializer;
use de::EnumAccess;
use de::IntoDeserializer;
use de::Visitor;
use serde::de;
use serde::de::value::Error;
use serde::de::value::SeqDeserializer;
use serde::forward_to_deserialize_any;

use crate::de::unit_only::UnitOnly;
use crate::types::HeaderValueRef;

#[derive(Debug)]
pub struct Value<'de>(pub HeaderValueRef<'de>);

impl<'de> Value<'de> {
    #[cfg(feature = "http")]
    #[inline]
    pub fn from_header_value(value: &'de http::HeaderValue) -> Self {
        Self(value.as_bytes())
    }
}

impl<'de> IntoDeserializer<'de> for Value<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_to_parsed_value {
    ($($ty:ident => $method:ident),* $(,)?) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: Visitor<'de>
            {
                str::from_utf8(self.0)
                    .map_err(de::Error::custom)?
                    .parse::<$ty>()
                    .map(|val| val.into_deserializer().$method(visitor))
                    .map_err(de::Error::custom)?
            }
        )*
    }
}

impl<'de> Deserializer<'de> for Value<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.0)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.0.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(iter::once(self)))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    forward_to_deserialize_any! {
        char
        str
        string
        bytes
        byte_buf
        unit
        unit_struct
        tuple
        tuple_struct
        map
        struct
        identifier
        ignored_any
    }

    forward_to_parsed_value! {
        bool => deserialize_bool,
        i8 => deserialize_i8,
        i16 => deserialize_i16,
        i32 => deserialize_i32,
        i64 => deserialize_i64,
        i128 => deserialize_i128,
        u8 => deserialize_u8,
        u16 => deserialize_u16,
        u32 => deserialize_u32,
        u64 => deserialize_u64,
        u128 => deserialize_u128,
        f32 => deserialize_f32,
        f64 => deserialize_f64,
    }
}

impl<'de> EnumAccess<'de> for Value<'de> {
    type Error = Error;
    type Variant = UnitOnly<Self::Error>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self).map(UnitOnly::new)
    }
}
