use derive_more::From;
use serde::Serialize;

use crate::types::DeleteMarkerEntry;
use crate::types::ObjectVersion;

#[derive(Debug, From, Serialize)]
pub enum DeleteMarkerOrVersion {
    DeleteMarker(DeleteMarkerEntry),
    Version(ObjectVersion),
}
