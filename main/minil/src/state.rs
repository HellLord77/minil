use axum::extract::FromRef;
use derive_more::Constructor;
use sea_orm::DbConn;

#[derive(Debug, Clone, Constructor, FromRef)]
pub(crate) struct AppState {
    pub(crate) db_conn: DbConn,
}
