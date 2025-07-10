macro_rules! app_define_routes {
    ($router:ident {
        $($path:expr => $method:ident($handler:expr)),* $(,)?
    }) => {
        $router$(.route(::axum_extra::vpath!($path), ::axum::routing::$method($handler)))*
    };
}

macro_rules! app_define_handler {
    ($handler_fn:ident {
        $($ck_ty:ident => $handler:ident,)*
        _ => $def_handler:ident
    }) => {
        #[allow(non_snake_case)]
        async fn $handler_fn(
            $($ck_ty: ::std::option::Option<::axum_s3::operation::check::$ck_ty>,)*
            ::axum::extract::State(state): ::axum::extract::State<$crate::state::AppState>,
            request: ::axum::extract::Request,
        ) -> ::axum::response::Response {
            $(if $ck_ty.is_some() {
                ::axum::handler::Handler::call($handler, request, state).await
            } else )* {
                ::axum::handler::Handler::call($def_handler, request, state).await
            }
        }
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
                ::tracing::error!(%err, "{}", ::std::stringify!($var));
            })*
            _ => {}
        }
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

macro_rules! app_output_err {
    ($expr:expr) => {
        match $expr {
            output => {
                ::std::dbg!(&output);
                ::axum::response::IntoResponse::into_response(output)
            }
        }
    };
}

macro_rules! app_response_err {
    (($err:expr, $parts:expr) {
        $($var:ident => $out_ty:ident),* $(,)?
        _ => !
        $($var_err:ident => $status:ident),* $(,)?
    }) => {
        match $err {
            $(Self::$var => {
                $crate::macros::app_output_err!(::axum_s3::error::$out_ty::from($parts))
            },)*
            $(Self::$var_err => $crate::macros::app_output_err!(StatusCode::$status),)*
        }
    };
}

pub(crate) use app_define_handler;
pub(crate) use app_define_routes;
pub(crate) use app_ensure_eq;
pub(crate) use app_ensure_matches;
pub(crate) use app_log_err;
pub(crate) use app_output;
pub(crate) use app_output_err;
pub(crate) use app_response_err;
