use std::env;
use std::future;
use std::net::Ipv4Addr;
use std::time::Instant;

use axum::Router;
use axum::debug_handler;
use axum::debug_middleware;
use axum::extract::FromRef;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::put;
use axum_extra::vpath;
use axum_s3::CreateBucketInput;
use axum_s3::CreateBucketOutput;
use axum_s3::DeleteBucketInput;
use axum_s3::ListBucketsInput;
use axum_s3::ListBucketsOutput;
use axum_s3::ListObjectsInput;
use serde_s3::operation::CreateBucketOutputHeader;
use serde_s3::operation::ListBucketsOutputBody;
use sqlx::Pool;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::migrate;
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::request_id::MakeRequestUuid;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const PROCESS_TIME: &str = "X-Process-Time";

#[derive(Debug, Clone, FromRef)]
struct State {
    db_pool: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .expect("unable to install global subscriber");

    let db_conn_str = env::var("DATABASE_URL").unwrap_or_else(|_err| "sqlite::memory:".to_owned());
    tracing::info!("connecting to {}", db_conn_str);
    let db_pool = SqlitePool::connect(&db_conn_str)
        .await
        .expect("failed to connect to database");
    migrate!()
        .run(&db_pool)
        .await
        .expect("failed to run migrations");

    let state = State { db_pool };
    let middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .propagate_x_request_id()
        .insert_response_header_if_not_present(header::SERVER, HeaderValue::from_static("axum"))
        .decompression()
        .compression()
        .trace_for_http();
    let app = Router::new()
        .route(
            vpath!("/{bucket}"),
            put(create_bucket).delete(delete_bucket).get(list_objects),
        )
        .route(vpath!("/"), get(list_buckets))
        .with_state(state)
        .layer(axum::middleware::from_fn(set_process_time))
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
async fn set_process_time(request: Request, next: Next) -> Response {
    let start_time = Instant::now();
    let mut response = next.run(request).await;

    if !response.headers().contains_key(PROCESS_TIME) {
        let process_time = start_time.elapsed().as_secs_f64().to_string();
        response
            .headers_mut()
            .insert(PROCESS_TIME, process_time.parse().unwrap());
    }
    response
}

#[debug_handler]
async fn create_bucket(input: CreateBucketInput) -> impl IntoResponse {
    dbg!(&input);
    let location = format!("/{}", input.bucket());
    let header = CreateBucketOutputHeader { location };
    CreateBucketOutput { header }
}

#[debug_handler]
async fn delete_bucket(input: DeleteBucketInput) -> impl IntoResponse {
    dbg!(&input);
    StatusCode::NO_CONTENT
}

#[debug_handler]
async fn list_buckets(input: ListBucketsInput) -> impl IntoResponse {
    dbg!(&input);
    let body = ListBucketsOutputBody {
        buckets: vec![],
        owner: None,
        continuation_token: None,
        prefix: None,
    };
    ListBucketsOutput { body }
}

#[debug_handler]
async fn list_objects(input: ListObjectsInput) -> impl IntoResponse {
    dbg!(&input);
    ()
}
