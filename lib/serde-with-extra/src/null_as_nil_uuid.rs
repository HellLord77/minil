use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::IntoDeserializer;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;
use uuid::Uuid;

const NULL: &str = "null";
const NIL_UUID: Uuid = Uuid::nil();

pub struct NullAsNilUuid;

impl SerializeAs<Uuid> for NullAsNilUuid {
    fn serialize_as<S>(source: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *source {
            NIL_UUID => serializer.serialize_str(NULL),
            source => source.serialize(serializer),
        }
    }
}

impl<'de> DeserializeAs<'de, Uuid> for NullAsNilUuid {
    fn deserialize_as<D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            NULL => Ok(NIL_UUID),
            str => Uuid::deserialize(str.into_deserializer()),
        }
    }
}
