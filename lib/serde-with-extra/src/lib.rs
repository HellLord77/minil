mod display_from_bytes;

#[cfg(feature = "http-range")]
mod http_range;

#[cfg(feature = "query")]
mod query;

#[cfg(feature = "uuid")]
mod null_as_nil_uuid;

pub use display_from_bytes::DisplayFromBytes;
use serde_with::DisplayFromStr;
use serde_with::IfIsHumanReadable;

#[rustfmt::skip]
#[cfg(feature = "http-range")]
pub use http_range::SerdeHttpRange;

#[rustfmt::skip]
#[cfg(feature = "query")]
pub use query::SerdeQuery;

#[rustfmt::skip]
#[cfg(feature = "uuid")]
pub use null_as_nil_uuid::NullAsNilUuid;

pub type DisplayFromUtf8 = IfIsHumanReadable<DisplayFromStr, DisplayFromBytes>;
