use std::marker::PhantomData;

use de::DeserializeSeed;
use de::VariantAccess;
use de::Visitor;
use serde::de;
use serde::de::Unexpected;

pub struct UnitOnly<E> {
    marker: PhantomData<E>,
}

impl<E> UnitOnly<E> {
    #[inline]
    pub fn new<T>(t: T) -> (T, Self) {
        (
            t,
            UnitOnly {
                marker: PhantomData,
            },
        )
    }
}

impl<'de, E> VariantAccess<'de> for UnitOnly<E>
where
    E: de::Error,
{
    type Error = E;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}
