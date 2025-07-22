use http_content_range::ContentRangeBytes;
use http_range_header::ParsedRanges;
use http_range_header::parse_range_header;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;
use serde::de;
use serde_with::DeserializeAs;
use serde_with::SerializeAs;

pub struct SerdeHttpRange;

impl SerializeAs<ContentRangeBytes> for SerdeHttpRange {
    fn serialize_as<S>(source: &ContentRangeBytes, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!(
            "bytes {}-{}/{}",
            source.first_byte, source.last_byte, source.complete_length
        ))
    }
}

impl<'de> DeserializeAs<'de, ParsedRanges> for SerdeHttpRange {
    fn deserialize_as<D>(deserializer: D) -> Result<ParsedRanges, D::Error>
    where
        D: Deserializer<'de>,
    {
        parse_range_header(&String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}
