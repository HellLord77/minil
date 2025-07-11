use std::sync::Arc;

use sea_orm::DatabaseTransaction;

pub(crate) type DbTxn = Arc<DatabaseTransaction>;
