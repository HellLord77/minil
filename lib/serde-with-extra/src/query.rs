use std::fmt;
use std::marker::PhantomData;

use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de;
use serde::de::DeserializeOwned;
use serde::de::Visitor;
use serde::ser;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

pub struct SerdeQuery;

impl<T> SerializeAs<T> for SerdeQuery
where
    T: Serialize,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[cfg(feature = "query-multi")]
        let ser = serde_html_form::to_string(source);
        #[cfg(not(feature = "query-multi"))]
        let ser = serde_urlencoded::to_string(source);

        serializer.serialize_str(&ser.map_err(ser::Error::custom)?)
    }
}

impl<'de, T> DeserializeAs<'de, T> for SerdeQuery
where
    T: DeserializeOwned,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Helper<S>(PhantomData<S>);

        impl<S> Visitor<'_> for Helper<S>
        where
            S: DeserializeOwned,
        {
            type Value = S;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a byte array")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                #[cfg(feature = "query-multi")]
                let de = serde_html_form::from_str(v);
                #[cfg(not(feature = "query-multi"))]
                let de = serde_urlencoded::from_str(v);

                de.map_err(de::Error::custom)
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                #[cfg(feature = "query-multi")]
                let de = serde_html_form::from_bytes(v);
                #[cfg(not(feature = "query-multi"))]
                let de = serde_urlencoded::from_bytes(v);

                de.map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_bytes(Helper(PhantomData))
    }
}
