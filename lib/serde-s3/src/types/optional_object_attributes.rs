use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum OptionalObjectAttributes {
    RestoreStatus,
}
