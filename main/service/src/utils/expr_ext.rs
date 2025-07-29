use sea_orm::prelude::*;
use sea_orm::*;
use sea_query::*;

pub(crate) trait ExprExt {
    fn null() -> Expr;
}

impl ExprExt for Expr {
    fn null() -> Expr {
        Expr::expr(Keyword::Null)
    }
}
