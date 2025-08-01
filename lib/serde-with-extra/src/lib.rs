mod as_string;
mod display_from_bytes;

#[cfg(feature = "http-range")]
mod http_range;

pub use as_string::AsString;
pub use display_from_bytes::DisplayFromBytes;
#[cfg(feature = "http-range")]
pub use http_range::SerdeHttpRange;
use serde_with::DisplayFromStr;
use serde_with::IfIsHumanReadable;

pub type DisplayFromUtf8 = IfIsHumanReadable<DisplayFromStr, DisplayFromBytes>;
