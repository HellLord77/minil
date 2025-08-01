use core::fmt;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::Deserializer;
use serde::Serializer;
use serde::de::Error as DeError;
use serde::de::Visitor;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

pub struct DisplayFromBytes;

impl<T> SerializeAs<T> for DisplayFromBytes
where
    T: Display,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(source.to_string().as_bytes())
    }
}

impl<'de, T> DeserializeAs<'de, T> for DisplayFromBytes
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Helper<S>(PhantomData<S>);
        impl<S> Visitor<'_> for Helper<S>
        where
            S: FromStr,
            <S as FromStr>::Err: Display,
        {
            type Value = S;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                str::from_utf8(v)
                    .map_err(DeError::custom)?
                    .parse::<Self::Value>()
                    .map_err(DeError::custom)
            }
        }

        deserializer.deserialize_bytes(Helper(PhantomData))
    }
}
