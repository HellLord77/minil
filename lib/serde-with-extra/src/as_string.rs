use std::marker::PhantomData;

use serde::Deserialize;
use serde::Deserializer;
use serde::de::IntoDeserializer;
use serde_with::DeserializeAs;

pub struct AsString<D>(PhantomData<D>);

impl<'de, T, TD> DeserializeAs<'de, T> for AsString<TD>
where
    TD: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        TD::deserialize_as(String::deserialize(deserializer)?.into_deserializer())
    }
}
