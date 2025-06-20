mod make_request_id;
mod state;

use std::collections::HashMap;
use std::env;
use std::future;
use std::net::Ipv4Addr;
use std::time::Instant;

use axum::Router;
use axum::ServiceExt;
use axum::debug_handler;
use axum::debug_middleware;
use axum::extract::FromRequest;
use axum::extract::Request;
use axum::extract::State;
use axum::http::HeaderName;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::put;
use axum_extra::extract::Query;
use axum_extra::vpath;
use axum_s3::CreateBucketInput;
use axum_s3::CreateBucketOutput;
use axum_s3::DeleteBucketInput;
use axum_s3::ListBucketsInput;
use axum_s3::ListBucketsOutput;
use axum_s3::ListObjectsInput;
use axum_s3::ListObjectsOutput;
use axum_s3::ListObjectsV2Input;
use axum_s3::ListObjectsV2Output;
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::Database;
use sea_orm::DbConn;
use serde_s3::operation::CreateBucketOutputHeader;
use serde_s3::operation::ListBucketsOutputBody;
use serde_s3::operation::ListObjectsOutputBody;
use serde_s3::operation::ListObjectsOutputHeader;
use serde_s3::operation::ListObjectsV2OutputBody;
use serde_s3::operation::ListObjectsV2OutputHeader;
use service::owner::Query as OwnerQuery;
use sha2::Digest;
use sha2::Sha256;
use tokio::net::TcpListener;
use tokio::signal;
use tower::Layer;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::normalize_path::NormalizePathLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::make_request_id::AppMakeRequestId;
use crate::state::AppState;

const NODE_NAME: &str = "minil";
const PROCESS_TIME: &str = "x-process-time";

const NODE_ID_HEADER: HeaderName = HeaderName::from_static("x-amz-id-2");
const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-amz-request-id");

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

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_err| "sqlite::memory:".to_owned());
    tracing::info!("connecting to {}", db_url);
    let db_conn = Database::connect(db_url)
        .await
        .expect("failed to connect to database");
    Migrator::up(&db_conn, None)
        .await
        .expect("failed to run migrations");

    let state = AppState { db_conn };
    let node_id = format!("{:x}", Sha256::digest(NODE_NAME.as_bytes()));
    let server = format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let middleware = ServiceBuilder::new()
        .set_request_id(REQUEST_ID_HEADER, AppMakeRequestId)
        .propagate_request_id(REQUEST_ID_HEADER)
        .override_response_header(
            NODE_ID_HEADER,
            node_id.parse::<HeaderValue>().expect("invalid node id"),
        )
        .insert_response_header_if_not_present(
            header::SERVER,
            server
                .parse::<HeaderValue>()
                .expect("invalid server header"),
        )
        .decompression()
        .compression()
        .trace_for_http();

    let router = Router::new()
        .route(
            vpath!("/{bucket}"),
            put(create_bucket)
                .delete(delete_bucket)
                .get(list_objects_handler),
        )
        .route(vpath!("/"), get(list_buckets))
        .with_state(state)
        .layer(axum::middleware::from_fn(set_process_time))
        .layer(middleware);

    let app = ServiceExt::<Request>::into_make_service(
        NormalizePathLayer::trim_trailing_slash().layer(router),
    );

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
        response.headers_mut().insert(
            PROCESS_TIME,
            process_time.parse().expect("invalid process time"),
        );
    }

    response
}

#[debug_handler]
async fn create_bucket(input: CreateBucketInput) -> impl IntoResponse {
    dbg!(&input);
    let location = format!("/{}", input.bucket);
    let header = CreateBucketOutputHeader { location };
    CreateBucketOutput { header }
}

#[debug_handler]
async fn delete_bucket(input: DeleteBucketInput) -> impl IntoResponse {
    dbg!(&input);
    StatusCode::NO_CONTENT
}

#[debug_handler]
async fn list_buckets(State(db): State<DbConn>, input: ListBucketsInput) -> impl IntoResponse {
    dbg!(&input);
    let owner = OwnerQuery::find_by_name(&db, "minil")
        .await
        .expect("failed to find owner")
        .expect("owner not found");
    dbg!(&owner);
    let body = ListBucketsOutputBody {
        buckets: vec![],
        owner: None,
        continuation_token: None,
        prefix: None,
    };
    ListBucketsOutput { body }
}

#[debug_handler]
async fn list_objects_handler(
    Query(query): Query<HashMap<String, String>>,
    state: State<AppState>,
    request: Request,
) -> Result<Response, Response> {
    if matches!(query.get("list-type"), Some(value) if value == "2") {
        let input = ListObjectsV2Input::from_request(request, &state).await?;
        Ok(list_objects_v2(input).await.into_response())
    } else {
        let input = ListObjectsInput::from_request(request, &state).await?;
        Ok(list_objects(input).await.into_response())
    }
}

#[debug_handler]
async fn list_objects(input: ListObjectsInput) -> impl IntoResponse {
    dbg!(&input);
    let header = ListObjectsOutputHeader {
        request_charged: None,
    };
    let body = ListObjectsOutputBody {
        common_prefixes: vec![],
        contents: vec![],
        delimiter: None,
        encoding_type: None,
        is_truncated: false,
        marker: "".to_string(),
        max_keys: 0,
        name: "".to_string(),
        next_marker: None,
        prefix: "".to_string(),
    };
    ListObjectsOutput { header, body }
}

#[debug_handler]
async fn list_objects_v2(input: ListObjectsV2Input) -> impl IntoResponse {
    dbg!(&input);
    let header = ListObjectsV2OutputHeader {
        request_charged: None,
    };
    let body = ListObjectsV2OutputBody {
        common_prefixes: vec![],
        contents: vec![],
        continuation_token: None,
        delimiter: None,
        encoding_type: None,
        is_truncated: false,
        key_count: 0,
        max_keys: 0,
        name: "".to_string(),
        next_continuation_token: None,
        prefix: "".to_string(),
        start_after: None,
    };
    ListObjectsV2Output { header, body }
}
