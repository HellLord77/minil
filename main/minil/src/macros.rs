macro_rules! app_ensure_eq {
    ($($arg:tt)*) => {
        ::ensure::ensure_eq!($($arg)*, $crate::error::AppError::NotImplemented);
    };
}

macro_rules! app_ensure_matches {
    ($($arg:tt)*) => {
        ::ensure::ensure_matches!($($arg)*, $crate::error::AppError::NotImplemented);
    };
}

macro_rules! app_output {
    ($expr:expr) => {
        match $expr {
            output => {
                ::std::dbg!(&output);
                ::std::result::Result::Ok(output)
            }
        }
    };
}

macro_rules! app_log_err {
    ($self:expr => $($variant:ident),* $(,)?) => {
        match $self {
            $(Self::$variant(err) => {
                ::tracing::error!(%err, "{}", ::std::stringify!($variant));
            })*
            _ => {}
        }
    };
}

macro_rules! app_err_output {
    ($expr:expr) => {
        match $expr {
            output => {
                ::std::dbg!(&output);
                ::axum::response::IntoResponse::into_response(output)
            }
        }
    };
}

pub(crate) use app_ensure_eq;
pub(crate) use app_ensure_matches;
pub(crate) use app_err_output;
pub(crate) use app_log_err;
pub(crate) use app_output;
