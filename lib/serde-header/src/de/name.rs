use std::hash::Hash;
use std::hash::Hasher;

use de::Visitor;
use serde::Deserializer;
use serde::de;
use serde::de::DeserializeSeed;
use serde::de::EnumAccess;
use serde::de::IntoDeserializer;
use serde::de::value::Error;
use serde::forward_to_deserialize_any;

use crate::de::unit_only::UnitOnly;
use crate::types::HeaderNameRef;

#[derive(Debug, Eq)]
pub struct Name<'de>(pub HeaderNameRef<'de>);

impl<'de> PartialEq for Name<'de> {
    #[cfg(feature = "unicase")]
    #[inline]
    fn eq(&self, other: &Name<'de>) -> bool {
        unicase::eq(self.0, other.0)
    }

    #[cfg(not(feature = "unicase"))]
    #[inline]
    fn eq(&self, other: &Name<'de>) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl<'de> Hash for Name<'de> {
    #[cfg(feature = "unicase")]
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        unicase::UniCase::new(&self.0).hash(state)
    }

    #[cfg(not(feature = "unicase"))]
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state)
    }
}

impl<'de> IntoDeserializer<'de> for Name<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<'de> Deserializer<'de> for Name<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.0)
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
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
        unit
        unit_struct
        newtype_struct
        seq
        tuple
        tuple_struct
        map
        struct
        identifier
        ignored_any
    }
}

impl<'de> EnumAccess<'de> for Name<'de> {
    type Error = Error;
    type Variant = UnitOnly<Self::Error>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self).map(UnitOnly::new)
    }
}
