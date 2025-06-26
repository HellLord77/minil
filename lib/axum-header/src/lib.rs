#[cfg(any(
    not(any(feature = "single", feature = "multi")),
    all(feature = "single", feature = "multi")
))]
compile_error!("expected one of single or multi");

mod error;
mod header;
mod rejection;

pub use error::TryIntoHeaderError;
pub use error::TryIntoHeaderErrorKind;
pub use header::Header;
pub use rejection::HeaderRejection;
