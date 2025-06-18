use axum::extract::FromRef;
use sea_orm::DbConn;

#[derive(Debug, Clone, FromRef)]
pub(crate) struct AppState {
    pub(crate) db_conn: DbConn,
}
