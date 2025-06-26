mod error;
mod make_request_id;
mod state;

use std::env;
use std::future;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::time::Instant;

use axum::Router;
use axum::ServiceExt;
use axum::extract::FromRequest;
use axum::extract::FromRequestParts;
use axum::extract::Request;
use axum::extract::State;
use axum::http::HeaderName;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::head;
use axum::routing::put;
use axum_extra::extract::Query;
use axum_extra::extract::QueryRejection;
use axum_extra::vpath;
use axum_s3::error::BucketAlreadyExistsResponse;
use axum_s3::error::BucketAlreadyOwnedByYouResponse;
use axum_s3::error::NoSuchBucketResponse;
use axum_s3::operation::CreateBucketInput;
use axum_s3::operation::CreateBucketOutput;
use axum_s3::operation::DeleteBucketInput;
use axum_s3::operation::DeleteBucketOutput;
use axum_s3::operation::HeadBucketInput;
use axum_s3::operation::HeadBucketOutput;
use axum_s3::operation::ListBucketsInput;
use axum_s3::operation::ListBucketsOutput;
use axum_s3::operation::ListObjectsInput;
use axum_s3::operation::ListObjectsOutput;
use axum_s3::operation::ListObjectsV2Input;
use axum_s3::operation::ListObjectsV2Output;
use ensure::ensure_eq;
use ensure::ensure_matches;
use minil_migration::Migrator;
use minil_migration::MigratorTrait;
use minil_service::BucketMutation;
use minil_service::BucketQuery;
use minil_service::OwnerQuery;
use sea_orm::Database;
use sea_orm::DbConn;
use serde_s3::operation::CreateBucketOutputHeader;
use serde_s3::operation::HeadBucketOutputHeader;
use serde_s3::operation::ListBucketsOutputBody;
use serde_s3::operation::ListObjectsOutputBody;
use serde_s3::operation::ListObjectsOutputHeader;
use serde_s3::operation::ListObjectsV2OutputBody;
use serde_s3::operation::ListObjectsV2OutputHeader;
use serde_s3::types::Bucket;
use serde_s3::types::Owner;
use serde_s3::utils::ListType2;
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
use unwrap_infallible::UnwrapInfallible;

use crate::error::AppError;
use crate::error::AppErrorDiscriminants;
use crate::error::AppResult;
use crate::make_request_id::AppMakeRequestId;
use crate::state::AppState;

const NODE_NAME: &str = "minil";
const NODE_REGION: &str = "minil";
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

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_err| unimplemented!());
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
        .decompression()
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
        .compression()
        .trace_for_http();

    let router = Router::new()
        .route(vpath!("/{bucket}"), get(list_objects_handler))
        .route(vpath!("/{bucket}"), put(create_bucket))
        .route(vpath!("/{bucket}"), delete(delete_bucket))
        .route(vpath!("/{bucket}"), head(head_bucket))
        .route(vpath!("/"), get(list_buckets))
        .with_state(state)
        .layer(middleware)
        .layer(middleware::from_fn(handle_app_error))
        .layer(middleware::from_fn(set_process_time));

    let app = ServiceExt::<Request>::into_make_service(
        NormalizePathLayer::trim_trailing_slash().layer(router),
    );

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000);
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    tracing::info!("listening on {}", addr);
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

async fn handle_app_error(request: Request, next: Next) -> Response {
    let (mut parts, body) = request.into_parts();
    let request = Request::from_parts(parts.clone(), body);
    let response = next.run(request).await;

    match response.extensions().get::<AppErrorDiscriminants>() {
        Some(err) => match err {
            AppErrorDiscriminants::BucketAlreadyExists => {
                BucketAlreadyExistsResponse::from_request_parts(&mut parts, &())
                    .await
                    .unwrap_infallible()
                    .into_response()
            }
            AppErrorDiscriminants::BucketAlreadyOwnedByYou => {
                BucketAlreadyOwnedByYouResponse::from_request_parts(&mut parts, &())
                    .await
                    .unwrap_infallible()
                    .into_response()
            }
            AppErrorDiscriminants::NoSuchBucket => {
                NoSuchBucketResponse::from_request_parts(&mut parts, &())
                    .await
                    .unwrap_infallible()
                    .into_response()
            }

            AppErrorDiscriminants::Forbidden => StatusCode::FORBIDDEN.into_response(),
            AppErrorDiscriminants::NotImplemented => StatusCode::NOT_IMPLEMENTED.into_response(),

            AppErrorDiscriminants::DbErr => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
        None => response,
    }
}

async fn create_bucket(
    State(db): State<DbConn>,
    input: CreateBucketInput,
) -> AppResult<CreateBucketOutput> {
    let owner = OwnerQuery::find_by_unique_id(&db, "minil").await?.unwrap();

    dbg!(&input);
    ensure_matches!(input.header.acl, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.bucket_object_lock_enabled,
        None | Some(false),
        AppError::NotImplemented
    );
    ensure_eq!(
        input.header.grant_full_control,
        None,
        AppError::NotImplemented
    );
    ensure_eq!(input.header.grant_read, None, AppError::NotImplemented);
    ensure_eq!(input.header.grant_read_acp, None, AppError::NotImplemented);
    ensure_eq!(input.header.grant_write, None, AppError::NotImplemented);
    ensure_eq!(input.header.grant_write_acp, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.object_ownership,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(input.body, None, AppError::NotImplemented);

    let bucket = BucketMutation::create(&db, owner.id, &input.bucket, NODE_REGION)
        .await?
        .ok_or(AppError::BucketAlreadyOwnedByYou)?;

    let output = CreateBucketOutput::builder()
        .header(
            CreateBucketOutputHeader::builder()
                .location(format!("/{}", bucket.name))
                .build(),
        )
        .build();

    dbg!(&output);
    Ok(output)
}

async fn delete_bucket(
    State(db): State<DbConn>,
    input: DeleteBucketInput,
) -> AppResult<DeleteBucketOutput> {
    let owner = OwnerQuery::find_by_unique_id(&db, "minil").await?.unwrap();

    dbg!(&input);

    if let Some(expected_bucket_owner) = input.header.expected_bucket_owner {
        if expected_bucket_owner != owner.name {
            return Err(AppError::Forbidden);
        }
    }
    BucketMutation::delete_by_unique_id(&db, owner.id, &input.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    let output = DeleteBucketOutput::builder().build();

    dbg!(&output);
    Ok(output)
}

async fn head_bucket(
    State(db): State<DbConn>,
    input: HeadBucketInput,
) -> AppResult<HeadBucketOutput> {
    let owner = OwnerQuery::find_by_unique_id(&db, "minil").await?.unwrap();

    dbg!(&input);

    if let Some(expected_bucket_owner) = input.header.expected_bucket_owner {
        if expected_bucket_owner != owner.name {
            return Err(AppError::Forbidden);
        }
    }
    let bucket = BucketQuery::find_by_unique_id(&db, owner.id, &input.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    let output = HeadBucketOutput::builder()
        .header(
            HeadBucketOutputHeader::builder()
                .bucket_region(bucket.region)
                .build(),
        )
        .build();

    dbg!(&output);
    Ok(output)
}

async fn list_buckets(
    State(db): State<DbConn>,
    input: ListBucketsInput,
) -> AppResult<ListBucketsOutput> {
    let owner = OwnerQuery::find_by_unique_id(&db, "minil").await?.unwrap();

    dbg!(&input);
    ensure_eq!(input.query.prefix, None, AppError::NotImplemented);

    let limit = input.query.max_buckets + 1;
    let mut buckets = BucketQuery::find_all_by_owner_id(
        &db,
        owner.id,
        input.query.prefix.as_deref(),
        input.query.continuation_token.as_deref(),
        Some(limit as u64),
    )
    .await?;
    let continuation_token = if buckets.len() == limit as usize {
        Some(buckets.pop().unwrap_or_else(|| unreachable!()).name)
    } else {
        None
    };

    let output = ListBucketsOutput::builder()
        .body(
            ListBucketsOutputBody::builder()
                .buckets(
                    buckets
                        .into_iter()
                        .map(|bucket| {
                            Bucket::builder()
                                .name(bucket.name)
                                .bucket_region(bucket.region)
                                .creation_date(bucket.created_at)
                                .build()
                        })
                        .collect::<Vec<_>>(),
                )
                .owner(
                    Owner::builder()
                        .display_name(owner.name)
                        .id(owner.id)
                        .build(),
                )
                .maybe_continuation_token(continuation_token)
                .maybe_prefix(input.query.prefix)
                .build(),
        )
        .build();

    dbg!(&output);
    Ok(output)
}

async fn list_objects_handler(
    list_type_2: Result<Query<ListType2>, QueryRejection>,
    state: State<AppState>,
    request: Request,
) -> Result<Response, Response> {
    if list_type_2.is_err() {
        let input = ListObjectsInput::from_request(request, &state).await?;
        Ok(list_objects(input).await.into_response())
    } else {
        let input = ListObjectsV2Input::from_request(request, &state).await?;
        Ok(list_objects_v2(input).await.into_response())
    }
}

async fn list_objects(input: ListObjectsInput) -> impl IntoResponse {
    dbg!(&input);

    let output = ListObjectsOutput::builder()
        .header(ListObjectsOutputHeader::builder().build())
        .body(
            ListObjectsOutputBody::builder()
                .common_prefixes(vec![])
                .contents(vec![])
                .is_truncated(false)
                .marker("".to_owned())
                .max_keys(0)
                .name("".to_owned())
                .prefix("".to_owned())
                .build(),
        )
        .build();

    dbg!(&output);
    output
}

async fn list_objects_v2(input: ListObjectsV2Input) -> impl IntoResponse {
    dbg!(&input);

    let output = ListObjectsV2Output::builder()
        .header(ListObjectsV2OutputHeader::builder().build())
        .body(
            ListObjectsV2OutputBody::builder()
                .common_prefixes(vec![])
                .contents(vec![])
                .is_truncated(false)
                .key_count(0)
                .max_keys(0)
                .name("".to_owned())
                .prefix("".to_owned())
                .build(),
        )
        .build();

    dbg!(&output);
    output
}
