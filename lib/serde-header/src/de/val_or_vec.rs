use std::iter;
use std::mem;
use std::vec;

use serde::de;
use serde::de::Deserializer;
use serde::de::IntoDeserializer;
use serde::de::Visitor;
use serde::de::value::Error;
use serde::de::value::SeqDeserializer;

use crate::de::value;

#[derive(Debug)]
pub enum ValOrVec<'de> {
    Val(value::Value<'de>),
    Vec(Vec<value::Value<'de>>),
}

impl<'de> ValOrVec<'de> {
    pub fn push(&mut self, new_val: value::Value<'de>) {
        match self {
            Self::Val(_) => {
                let old_self = mem::replace(self, Self::Vec(Vec::with_capacity(2)));

                let old_val = match old_self {
                    Self::Val(v) => v,
                    _ => unreachable!(),
                };

                let vec = match self {
                    Self::Vec(v) => v,
                    _ => unreachable!(),
                };

                vec.push(old_val);
                vec.push(new_val);
            }
            Self::Vec(vec) => vec.push(new_val),
        }
    }

    fn deserialize_val<U, E, F>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(value::Value<'de>) -> Result<U, E>,
        E: de::Error,
    {
        match self {
            ValOrVec::Val(val) => f(val),
            ValOrVec::Vec(_) => Err(de::Error::custom("unsupported value")),
        }
    }
}

impl<'de> IntoIterator for ValOrVec<'de> {
    type Item = value::Value<'de>;
    type IntoIter = ValOrVecIterator<'de>;

    fn into_iter(self) -> Self::IntoIter {
        ValOrVecIterator::new(self)
    }
}

pub enum ValOrVecIterator<'de> {
    Val(iter::Once<value::Value<'de>>),
    Vec(vec::IntoIter<value::Value<'de>>),
}

impl<'de> ValOrVecIterator<'de> {
    fn new(vv: ValOrVec<'de>) -> Self {
        match vv {
            ValOrVec::Val(val) => Self::Val(iter::once(val)),
            ValOrVec::Vec(vec) => Self::Vec(vec.into_iter()),
        }
    }
}

impl<'de> Iterator for ValOrVecIterator<'de> {
    type Item = value::Value<'de>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ValOrVecIterator::Val(iter) => iter.next(),
            ValOrVecIterator::Vec(iter) => iter.next(),
        }
    }
}

impl<'de> IntoDeserializer<'de> for ValOrVec<'de> {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

macro_rules! forward_to_deserialize_val {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
                where V: Visitor<'de>
            {
                self.deserialize_val(move |val| val.$method(visitor))
            }
        )*
    }
}

impl<'de> Deserializer<'de> for ValOrVec<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Self::Val(val) => val.deserialize_any(visitor),
            Self::Vec(_) => self.deserialize_seq(visitor),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            ValOrVec::Val(val) => val.deserialize_option(visitor),
            ValOrVec::Vec(_) => visitor.visit_some(self),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_unit_struct(name, visitor))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_newtype_struct(name, visitor))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqDeserializer::new(self.into_iter()))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_tuple(len, visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_tuple_struct(name, len, visitor))
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_struct(name, fields, visitor))
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_val(move |val| val.deserialize_enum(name, variants, visitor))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    forward_to_deserialize_val! {
        deserialize_bool,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i128,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_u128,
        deserialize_f32,
        deserialize_f64,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_unit,
        deserialize_identifier,
        deserialize_map,
    }
}
