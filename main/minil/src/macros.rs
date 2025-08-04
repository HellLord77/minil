macro_rules! _app_db_ref {
    ($db:ident) => {
        let $db = ::std::convert::AsRef::as_ref(&$db);
    };
}

macro_rules! _app_output {
    ($expr:expr) => {
        match $expr {
            output => {
                ::std::dbg!(&output);
                ::core::result::Result::Ok(output)
            }
        }
    };
}

macro_rules! _app_validate_digest {
    ($left:expr, $right:expr) => {
        if let (::core::option::Option::Some(left), right) = ($left, $right) {
            let left = ::base64::prelude::BASE64_STANDARD
                .decode(left)
                .map_err(|_| $crate::error::AppError::InvalidDigest)?;
            if left.len() != right.len() {
                ::core::result::Result::Err($crate::error::AppError::InvalidDigest)?
            }
            if left != right {
                ::core::result::Result::Err($crate::error::AppError::BadDigest)?
            }
        }
    };
}

macro_rules! _app_define_handler {
    ($handler_fn:ident {
        $($ck_ty:ident => $handler:ident,)*
        _ => $def_handler:ident $(,)?
    }) => {
        async fn $handler_fn(
            $(::paste::paste!([<$ck_ty:snake>]): ::core::option::Option<::axum_s3::check::$ck_ty>,)*
            ::axum::extract::State(state): ::axum::extract::State<$crate::state::AppState>,
            request: ::axum::extract::Request,
        ) -> ::axum::response::Response {
            $(if ::paste::paste!([<$ck_ty:snake>]).is_some() {
                ::axum::handler::Handler::call($handler, request, state).await
            } else )* {
                ::axum::handler::Handler::call($def_handler, request, state).await
            }
        }
    };
}

macro_rules! app_define_handler {
    ($handler:path) => {
        $handler
    };
    ({$($handler:tt)*}) => {
        ::axum_filter_router::axum_filter_handler!($crate::state::AppState {$($handler)*})
    };
}

macro_rules! app_define_handlers {
    ($router:ident {
        $($method:ident($path:literal) => $handler:tt),* $(,)?
    }) => {
        $router$(.route(
            ::axum_extra::vpath!($path),
            ::axum::routing::$method($crate::macros::app_define_handler!($handler))
        ))*
    };
}

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

macro_rules! app_log_err {
    ($err:expr => [$($var:ident),* $(,)?]) => {
        match $err {
            $(Self::$var(err) => {
                ::tracing::error!(%err, "{}", ::core::stringify!($var));
            })*
            _ => {}
        }
    };
}

macro_rules! app_output_err {
    ($expr:expr) => {
        match $expr {
            output => {
                ::tracing::warn!("{output:?}");
                ::axum::response::IntoResponse::into_response(output)
            }
        }
    };
}

macro_rules! app_response_err {
    (($err:expr, $parts:expr) {
        $($var:ident => $out_ty:ident,)*
        _ => [$($var_err:ident),* $(,)?] $(,)?
    }) => {
        match $err {
            $(Self::$var => {
                $crate::macros::app_output_err!(::axum_s3::error::$out_ty::from($parts))
            },)*
            $(Self::$var_err => {
                $crate::macros::app_output_err!(::axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            },)*
        }
    };
}

macro_rules! app_validate_owner {
    ($left:expr, $right:expr) => {
        if let ::core::option::Option::Some(left) = $left {
            if left != $right {
                Err($crate::error::AppError::AccessDenied)?
            }
        }
    };
}

pub(crate) use app_define_handler;
pub(crate) use app_define_handlers;
pub(crate) use app_ensure_eq;
pub(crate) use app_ensure_matches;
pub(crate) use app_log_err;
pub(crate) use app_output_err;
pub(crate) use app_response_err;
pub(crate) use app_validate_owner;
