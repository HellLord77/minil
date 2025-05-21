mod error;

use axum::Router;
use axum::debug_handler;
use axum::debug_middleware;
use axum::extract::Request;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::put;
use axum_extra::extract::Query;
use axum_header::Header;
use axum_xml::Xml;
use serde_s3::create_bucket::CreateBucketConfiguration;
use serde_s3::create_bucket::CreateBucketHeader;
use serde_s3::delete_bucket::DeleteBucketHeader;
use serde_s3::delete_bucket::DeleteBucketQuery;
use serde_s3::list_buckets::ListAllMyBucketsResult;
use serde_s3::list_buckets::ListBucketsQuery;
use sqlx::SqlitePool;
use sqlx::migrate;
use std::env;
use std::future;
use std::net::Ipv4Addr;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::request_id::MakeRequestUuid;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub(crate) use crate::error::Error;
pub(crate) use crate::error::Result;

const X_PROCESS_TIME: &str = "X-Process-Time";

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .expect("Unable to install global subscriber");

    let db_connection_str =
        env::var("DATABASE_URL").unwrap_or_else(|_err| "sqlite::memory:".into());
    tracing::info!("connecting to {}", db_connection_str);
    let pool = SqlitePool::connect(&db_connection_str)
        .await
        .expect("failed to connect to database");
    migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id()
        .insert_response_header_if_not_present(header::SERVER, HeaderValue::from_static("axum"))
        .decompression()
        .compression()
        .trace_for_http();
    let app = Router::new()
        .route("/create-bucket", put(create_bucket))
        .route("/delete-bucket", delete(delete_bucket))
        .route("/list-buckets", get(list_buckets))
        .with_state(pool)
        .layer(axum::middleware::from_fn(set_x_process_time))
        .layer(middleware);

    let addr = (Ipv4Addr::UNSPECIFIED, 3000);
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[debug_middleware]
async fn set_x_process_time(request: Request, next: Next) -> Response {
    let start_time = Instant::now();
    let mut response = next.run(request).await;

    if !response.headers().contains_key(X_PROCESS_TIME) {
        let process_time = start_time.elapsed().as_secs_f64().to_string();
        response
            .headers_mut()
            .insert(X_PROCESS_TIME, process_time.parse().unwrap());
    }
    response
}

#[debug_handler]
async fn create_bucket(
    Header(header): Header<CreateBucketHeader>,
    Xml(body): Xml<CreateBucketConfiguration>,
) -> impl IntoResponse {
    dbg!(&header);
    dbg!(&body);
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, "us-east-1".parse().unwrap());
    (StatusCode::OK, headers)
}

#[debug_handler]
async fn delete_bucket(
    Query(query): Query<DeleteBucketQuery>,
    Header(header): Header<DeleteBucketHeader>,
) -> impl IntoResponse {
    dbg!(&query);
    dbg!(&header);
    StatusCode::NO_CONTENT
}

#[debug_handler]
async fn list_buckets(
    Query(query): Query<ListBucketsQuery>,
    State(_pool): State<SqlitePool>,
) -> impl IntoResponse {
    dbg!(&query);
    let response = ListAllMyBucketsResult::default();
    dbg!(&response);
    Xml(response)
}
