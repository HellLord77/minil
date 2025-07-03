use std::collections::HashMap;

use serde::de::DeserializeSeed;
use serde::de::MapAccess;
use serde::de::value::Error;

use crate::de::name::Name;

pub struct CaselessMap<'de, M> {
    inner: M,
    map: HashMap<Name<'de>, &'static str>,
}

impl<'de, M> CaselessMap<'de, M>
where
    M: MapAccess<'de, Error = Error>,
{
    #[inline]
    pub fn new<I>(m: M, i: I) -> Self
    where
        I: Iterator<Item = (Name<'de>, &'static str)>,
    {
        Self {
            inner: m,
            map: i.collect(),
        }
    }
}

impl<'de, M> MapAccess<'de> for CaselessMap<'de, M>
where
    M: MapAccess<'de, Error = Error>,
{
    type Error = M::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        self.inner
            .next_key::<&'de str>()?
            .map(|mut field| {
                if let Some(canonical_field) = self.map.get(&Name(field)) {
                    field = canonical_field;
                }

                seed.deserialize(Name(field))
            })
            .transpose()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.inner.next_value_seed(seed)
    }
}
