mod display_from_bytes;

#[cfg(feature = "http-range")]
mod http_range;

#[cfg(feature = "uuid")]
mod null_as_nil_uuid;

pub use display_from_bytes::DisplayFromBytes;

#[rustfmt::skip]
#[cfg(feature = "http-range")]
pub use http_range::SerdeHttpRange;

#[rustfmt::skip]
#[cfg(feature = "uuid")]
pub use null_as_nil_uuid::NullAsNilUuid;

pub type DisplayFromUtf8 =
    serde_with::IfIsHumanReadable<serde_with::DisplayFromStr, DisplayFromBytes>;
