mod database_transaction;
mod error;
mod macros;
mod make_request_id;
mod service_builder_ext;
mod state;

use std::convert::identity;
use std::env;
use std::future;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;

use axum::Extension;
use axum::Router;
use axum::ServiceExt;
use axum::body::Body;
use axum::extract::Request;
use axum::extract::State;
use axum::http::HeaderName;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;
use axum_s3::operation::CreateBucketInput;
use axum_s3::operation::CreateBucketOutput;
use axum_s3::operation::DeleteBucketInput;
use axum_s3::operation::DeleteBucketOutput;
use axum_s3::operation::DeleteObjectInput;
use axum_s3::operation::DeleteObjectOutput;
use axum_s3::operation::GetBucketLocationInput;
use axum_s3::operation::GetBucketLocationOutput;
use axum_s3::operation::GetBucketVersioningInput;
use axum_s3::operation::GetBucketVersioningOutput;
use axum_s3::operation::GetObjectInput;
use axum_s3::operation::GetObjectOutput;
use axum_s3::operation::HeadBucketInput;
use axum_s3::operation::HeadBucketOutput;
use axum_s3::operation::HeadObjectInput;
use axum_s3::operation::HeadObjectOutput;
use axum_s3::operation::ListBucketsInput;
use axum_s3::operation::ListBucketsOutput;
use axum_s3::operation::ListObjectsInput;
use axum_s3::operation::ListObjectsOutput;
use axum_s3::operation::ListObjectsV2Input;
use axum_s3::operation::ListObjectsV2Output;
use axum_s3::operation::PutBucketVersioningInput;
use axum_s3::operation::PutBucketVersioningOutput;
use axum_s3::operation::PutObjectInput;
use axum_s3::operation::PutObjectOutput;
use axum_s3::utils::ErrorParts;
use base64::Engine;
use digest::Digest;
use futures::StreamExt;
use futures::TryStreamExt;
use http_content_range::ContentRangeBytes;
use indexmap::IndexSet;
use minil_config::AppConfig;
use minil_migration::Migrator;
use minil_migration::MigratorTrait;
use minil_service::BucketMutation;
use minil_service::BucketQuery;
use minil_service::ChunkQuery;
use minil_service::ObjectMutation;
use minil_service::ObjectQuery;
use minil_service::OwnerQuery;
use minil_service::PartQuery;
use minil_service::VersionMutation;
use minil_service::VersionQuery;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DbConn;
use sea_orm::TransactionTrait;
use serde_s3::operation::CreateBucketOutputHeader;
use serde_s3::operation::DeleteObjectOutputHeader;
use serde_s3::operation::GetBucketLocationOutputBody;
use serde_s3::operation::GetBucketVersioningOutputBody;
use serde_s3::operation::GetObjectOutputHeader;
use serde_s3::operation::HeadBucketOutputHeader;
use serde_s3::operation::HeadObjectOutputHeader;
use serde_s3::operation::ListBucketsOutputBody;
use serde_s3::operation::ListObjectsOutputBody;
use serde_s3::operation::ListObjectsOutputHeader;
use serde_s3::operation::ListObjectsV2OutputBody;
use serde_s3::operation::ListObjectsV2OutputHeader;
use serde_s3::operation::PutObjectOutputHeader;
use serde_s3::types::Bucket;
use serde_s3::types::BucketLocationConstraint;
use serde_s3::types::BucketVersioningStatus;
use serde_s3::types::CommonPrefix;
use serde_s3::types::EncodingType;
use serde_s3::types::MfaDeleteStatus;
use serde_s3::types::Object;
use serde_s3::types::Owner;
use sha2::Sha256;
use tokio::net::TcpListener;
use tokio_util::io::StreamReader;
use tower::Layer;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::normalize_path::NormalizePathLayer;
use tracing::debug;
use tracing::info;
use tracing::instrument;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::prelude::*;

use crate::database_transaction::DbTxn;
use crate::error::AppError;
use crate::error::AppErrorDiscriminants;
use crate::error::AppResult;
use crate::macros::app_define_handler;
use crate::macros::app_define_routes;
use crate::macros::app_ensure_eq;
use crate::macros::app_ensure_matches;
use crate::macros::app_validate_digest;
use crate::macros::app_validate_owner;
use crate::make_request_id::AppMakeRequestId;
use crate::service_builder_ext::AppServiceBuilderExt;
use crate::state::AppState;

#[cfg(debug_assertions)]
#[global_allocator]
static ALLOCATOR: cap::Cap<std::alloc::System> = cap::Cap::new(
    std::alloc::System,
    bytesize::ByteSize::mib(64).as_u64() as usize,
);

const NODE_NAME: &str = "minil";
const PROCESS_TIME: &str = "x-process-time";

const NODE_ID_HEADER: HeaderName = HeaderName::from_static("x-amz-id-2");
const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-amz-request-id");

#[tokio::main]
async fn main() {
    let config = dbg!(init_config());
    let _log_guard = init_trace(&config);
    let db = init_db(&config).await;

    let state = AppState::new(db);
    let node_id = format!("{:x}", Sha256::digest(NODE_NAME.as_bytes()));
    let server = format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let middleware = ServiceBuilder::new()
        .trace_for_http()
        .decompression()
        // fixme .compression()
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
        .middleware_fn(set_process_time)
        .middleware_fn(handle_app_err)
        .middleware_fn_with_state(state.clone(), manage_db_txn);

    let router = Router::new();
    let router = app_define_routes!(router {
        "/" => get(list_buckets),

        "/{Bucket}" => delete(delete_bucket),
        "/{Bucket}" => get(get_bucket_handler),
        "/{Bucket}" => head(head_bucket),
        "/{Bucket}" => put(put_bucket_handler),

        "/{Bucket}/{*Key}" => delete(delete_object),
        "/{Bucket}/{*Key}" => get(get_object),
        "/{Bucket}/{*Key}" => head(head_object),
        "/{Bucket}/{*Key}" => put(put_object),
    })
    .method_not_allowed_fallback(async || AppError::MethodNotAllowed)
    .with_state(state)
    .layer(middleware);
    let app = ServiceExt::<Request>::into_make_service(
        NormalizePathLayer::trim_trailing_slash().layer(router),
    );

    let addr = config.server.to_socket();
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    info!("tcp listening on {}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn init_config() -> AppConfig {
    match env::var("RUST_ENV") {
        Ok(path) => dotenvy::from_path(path).expect("failed to load env file"),
        Err(_) => match dotenvy::dotenv() {
            Ok(path) => {
                info!("loaded env from {}", path.display());
            }
            Err(err) => {
                debug!(%err, "DotEnvyError")
            }
        },
    }

    AppConfig::try_new().expect("failed to load config")
}

fn init_trace(config: &AppConfig) -> WorkerGuard {
    let (writer, _guard) = config.log.stream.to_writer();
    tracing_subscriber::registry()
        .with(match EnvFilter::try_from_default_env() {
            Ok(filter) => filter.boxed(),
            Err(_) => Targets::new()
                .with_target(env!("CARGO_CRATE_NAME"), config.log.level.try_as_level())
                .boxed(),
        })
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(writer)
                .event_format(config.log.format.to_format()),
        )
        .init();

    _guard
}

async fn init_db(config: &AppConfig) -> DbConn {
    let mut options =
        ConnectOptions::new(config.database.try_to_url().expect("invalid database url"));
    options.sqlx_logging_level(config.database.log_level.as_filter());
    options.sqlx_slow_statements_logging_settings(
        config.database.slow_log_level.as_filter(),
        Duration::from_secs(config.database.slow_threshold),
    );

    let connection = Database::connect(options)
        .await
        .expect("failed to connect to database");
    Migrator::up(&connection, None)
        .await
        .expect("failed to run migrations");

    connection
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
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
    let elapsed = start_time.elapsed();

    let headers = response.headers_mut();
    if !headers.contains_key(PROCESS_TIME) {
        headers.insert(
            PROCESS_TIME,
            elapsed
                .as_secs_f64()
                .to_string()
                .parse()
                .expect("invalid process time"),
        );
    }

    response
}

async fn handle_app_err(request: Request, next: Next) -> Response {
    let (parts, body) = request.into_parts();
    let err_parts = ErrorParts::from(&parts);
    let request = Request::from_parts(parts, body);
    let mut response = next.run(request).await;

    let err = response.extensions_mut().remove::<AppErrorDiscriminants>();
    match err {
        Some(err) => err.into_response(err_parts),
        None => response,
    }
}

async fn manage_db_txn(
    State(db_conn): State<DbConn>,
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let db_txn = Arc::new(db_conn.begin().await?);
    request.extensions_mut().insert(Arc::clone(&db_txn));
    let response = next.run(request).await;

    let db_txn = Arc::into_inner(db_txn).expect("failed to take transaction");
    if response.status().is_success() {
        debug!("committing transaction");
        db_txn.commit().await?;
    } else {
        debug!("rolling back transaction");
        db_txn.rollback().await?;
    }

    Ok(response)
}

#[instrument(skip(db), ret)]
async fn list_buckets(
    Extension(db): Extension<DbTxn>,
    input: ListBucketsInput,
) -> AppResult<ListBucketsOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    let limit = input.query.max_buckets + 1;
    let mut buckets = BucketQuery::find_by_owner_id(
        db.as_ref(),
        owner.id,
        input.query.prefix.as_deref(),
        input.query.continuation_token.as_deref(),
        Some(limit as u64),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let continuation_token = (buckets.len() == limit as usize).then(|| {
        buckets.pop();
        buckets
            .last()
            .unwrap_or_else(|| unreachable!())
            .name
            .clone()
    });

    Ok(ListBucketsOutput::builder()
        .body(
            ListBucketsOutputBody::builder()
                .buckets(
                    buckets
                        .into_iter()
                        .map(|bucket| {
                            Bucket::builder()
                                .name(bucket.name)
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
        .build())
}

#[instrument(skip(db), ret)]
async fn delete_bucket(
    Extension(db): Extension<DbTxn>,
    input: DeleteBucketInput,
) -> AppResult<DeleteBucketOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    BucketMutation::delete(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    Ok(DeleteBucketOutput::builder().build())
}

app_define_handler!(get_bucket_handler {
    GetBucketVersioningCheck => get_bucket_versioning,
    GetBucketLocationCheck => get_bucket_location,
    ListObjectsV2Check => list_objects_v2,
    _ => list_objects,
});

#[instrument(skip(db), ret)]
async fn get_bucket_versioning(
    Extension(db): Extension<DbTxn>,
    input: GetBucketVersioningInput,
) -> AppResult<GetBucketVersioningOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let mfa_delete = bucket.mfa_delete.map(|mfa_delete| {
        if mfa_delete {
            MfaDeleteStatus::Enabled
        } else {
            MfaDeleteStatus::Disabled
        }
    });
    let status = bucket.versioning.map(|versioning| {
        if versioning {
            BucketVersioningStatus::Enabled
        } else {
            BucketVersioningStatus::Suspended
        }
    });

    Ok(GetBucketVersioningOutput::builder()
        .body(
            GetBucketVersioningOutputBody::builder()
                .maybe_mfa_delete(mfa_delete)
                .maybe_status(status)
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn get_bucket_location(
    Extension(db): Extension<DbTxn>,
    input: GetBucketLocationInput,
) -> AppResult<GetBucketLocationOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    Ok(GetBucketLocationOutput::builder()
        .body(
            GetBucketLocationOutputBody::builder()
                .content(BucketLocationConstraint::UsEast1)
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn list_objects_v2(
    Extension(db): Extension<DbTxn>,
    input: ListObjectsV2Input,
) -> AppResult<ListObjectsV2Output> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_matches!(input.header.optional_object_attributes, None);
    app_ensure_matches!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let limit = input.query.max_keys + 1;
    let (mut objects, versions) = ObjectQuery::find_version_by_bucket_id(
        db.as_ref(),
        bucket.id,
        input.query.prefix.as_deref(),
        input
            .query
            .continuation_token
            .as_deref()
            .or(input.query.start_after.as_deref()),
        Some(limit as u64),
    )
    .await?
    .try_collect::<(Vec<_>, Vec<_>)>()
    .await?;
    let next_continuation_token = (objects.len() == limit as usize).then(|| {
        objects.pop();
        objects.last().unwrap_or_else(|| unreachable!()).key.clone()
    });
    let mut common_prefixes = IndexSet::new();
    if let Some(delimiter) = &input.query.delimiter {
        let prefix_len = input.query.prefix.as_deref().map_or(0, str::len);
        let offset = prefix_len + delimiter.len();
        objects.retain(|object| {
            if let Some(delimiter_index) = &object.key[prefix_len..].find(delimiter) {
                common_prefixes.insert(object.key[..offset + delimiter_index].to_owned());
                false
            } else {
                true
            }
        });
    }
    let key_count = objects.len() + common_prefixes.len();
    let encode = if let Some(encoding_type) = &input.query.encoding_type {
        match encoding_type {
            EncodingType::Url => |string: String| urlencoding::encode(&string).to_string(),
        }
    } else {
        identity
    };

    Ok(ListObjectsV2Output::builder()
        .header(ListObjectsV2OutputHeader::builder().build())
        .body(
            ListObjectsV2OutputBody::builder()
                .common_prefixes(
                    common_prefixes
                        .into_iter()
                        .map(|common_prefix| {
                            CommonPrefix::builder()
                                .prefix(encode(common_prefix))
                                .build()
                        })
                        .collect(),
                )
                .contents(
                    objects
                        .into_iter()
                        .zip(versions.into_iter())
                        .map(|(object, version)| {
                            Object::builder()
                                .e_tag(version.e_tag)
                                .key(encode(object.key))
                                .last_modified(version.updated_at.unwrap_or(version.created_at))
                                .maybe_owner(input.query.fetch_owner.unwrap_or_default().then(
                                    || {
                                        Owner::builder()
                                            .display_name(owner.name.clone())
                                            .id(owner.id)
                                            .build()
                                    },
                                ))
                                .size(version.size as u64)
                                .build()
                        })
                        .collect(),
                )
                .maybe_continuation_token(input.query.continuation_token)
                .maybe_delimiter(input.query.delimiter.map(encode))
                .maybe_encoding_type(input.query.encoding_type)
                .is_truncated(next_continuation_token.is_some())
                .key_count(key_count as u16)
                .max_keys(input.query.max_keys)
                .name(bucket.name)
                .maybe_next_continuation_token(next_continuation_token)
                .prefix(input.query.prefix.map(encode).unwrap_or_default())
                .maybe_start_after(input.query.start_after.map(encode))
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn list_objects(
    Extension(db): Extension<DbTxn>,
    input: ListObjectsInput,
) -> AppResult<ListObjectsOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_matches!(input.header.optional_object_attributes, None);
    app_ensure_matches!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let limit = input.query.max_keys + 1;
    let (mut objects, versions) = ObjectQuery::find_version_by_bucket_id(
        db.as_ref(),
        bucket.id,
        input.query.prefix.as_deref(),
        input.query.marker.as_deref(),
        Some(limit as u64),
    )
    .await?
    .try_collect::<(Vec<_>, Vec<_>)>()
    .await?;
    let next_marker = (objects.len() == limit as usize).then(|| {
        objects.pop();
        objects.last().unwrap_or_else(|| unreachable!()).key.clone()
    });
    let mut common_prefixes = IndexSet::new();
    if let Some(delimiter) = &input.query.delimiter {
        let prefix_len = input.query.prefix.as_deref().map_or(0, str::len);
        let offset = prefix_len + delimiter.len();
        objects.retain(|object| {
            if let Some(delimiter_index) = &object.key[prefix_len..].find(delimiter) {
                common_prefixes.insert(object.key[..offset + delimiter_index].to_owned());
                false
            } else {
                true
            }
        });
    }
    let delimiter_is_some = input.query.delimiter.is_some();
    let encode = if let Some(encoding_type) = &input.query.encoding_type {
        match encoding_type {
            EncodingType::Url => |string: String| urlencoding::encode(&string).to_string(),
        }
    } else {
        identity
    };

    Ok(ListObjectsOutput::builder()
        .header(ListObjectsOutputHeader::builder().build())
        .body(
            ListObjectsOutputBody::builder()
                .common_prefixes(
                    common_prefixes
                        .into_iter()
                        .map(|common_prefix| {
                            CommonPrefix::builder()
                                .prefix(encode(common_prefix))
                                .build()
                        })
                        .collect(),
                )
                .contents(
                    objects
                        .into_iter()
                        .zip(versions.into_iter())
                        .map(|(object, version)| {
                            Object::builder()
                                .e_tag(version.e_tag)
                                .key(encode(object.key))
                                .last_modified(version.updated_at.unwrap_or(version.created_at))
                                .owner(
                                    Owner::builder()
                                        .display_name(owner.name.clone())
                                        .id(owner.id)
                                        .build(),
                                )
                                .size(version.size as u64)
                                .build()
                        })
                        .collect(),
                )
                .maybe_delimiter(input.query.delimiter.map(encode))
                .maybe_encoding_type(input.query.encoding_type)
                .is_truncated(next_marker.is_some())
                .marker(input.query.marker.map(encode).unwrap_or_default())
                .max_keys(input.query.max_keys)
                .name(bucket.name)
                .maybe_next_marker(
                    next_marker
                        .filter(|_next_marker| delimiter_is_some)
                        .map(encode),
                )
                .prefix(input.query.prefix.map(encode).unwrap_or_default())
                .build(),
        )
        .build())
}

app_define_handler!(put_bucket_handler {
    PutBucketVersioningCheck => put_bucket_versioning,
    _ => create_bucket,
});

#[instrument(skip(db), ret)]
async fn put_bucket_versioning(
    Extension(db): Extension<DbTxn>,
    input: PutBucketVersioningInput,
) -> AppResult<PutBucketVersioningOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_eq!(input.header.content_md5, None);
    app_ensure_eq!(input.header.mfa, None);
    app_ensure_matches!(input.header.sdk_checksum_algorithm, None);
    app_ensure_matches!(
        input.body.mfa_delete,
        None | Some(MfaDeleteStatus::Disabled)
    );

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let mfa_delete = input
        .body
        .mfa_delete
        .map(|mfa_delete| matches!(mfa_delete, MfaDeleteStatus::Enabled));
    let status = input
        .body
        .status
        .map(|status| matches!(status, BucketVersioningStatus::Enabled));
    BucketMutation::update_versioning(
        db.as_ref(),
        owner.id,
        &input.path.bucket,
        mfa_delete,
        status,
    )
    .await?
    .ok_or(AppError::NoSuchBucket)?;

    Ok(PutBucketVersioningOutput::builder().build())
}

#[instrument(skip(db), ret)]
async fn head_bucket(
    Extension(db): Extension<DbTxn>,
    input: HeadBucketInput,
) -> AppResult<HeadBucketOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let region = serde_plain::to_string(&BucketLocationConstraint::UsEast1)
        .unwrap_or_else(|_err| unreachable!());

    Ok(HeadBucketOutput::builder()
        .header(
            HeadBucketOutputHeader::builder()
                .bucket_region(region)
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn create_bucket(
    Extension(db): Extension<DbTxn>,
    input: CreateBucketInput,
) -> AppResult<CreateBucketOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_matches!(input.header.acl, None);
    app_ensure_matches!(input.header.bucket_object_lock_enabled, None | Some(false));
    app_ensure_eq!(input.header.grant_full_control, None);
    app_ensure_eq!(input.header.grant_read, None);
    app_ensure_eq!(input.header.grant_read_acp, None);
    app_ensure_eq!(input.header.grant_write, None);
    app_ensure_eq!(input.header.grant_write_acp, None);
    app_ensure_matches!(input.header.object_ownership, None);
    app_ensure_matches!(input.body, None);

    let bucket = BucketMutation::insert(db.as_ref(), owner.id, input.path.bucket)
        .await?
        .ok_or(AppError::BucketAlreadyOwnedByYou)?;

    Ok(CreateBucketOutput::builder()
        .header(
            CreateBucketOutputHeader::builder()
                .location(format!("/{}", bucket.name))
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn delete_object(
    Extension(db): Extension<DbTxn>,
    input: DeleteObjectInput,
) -> AppResult<DeleteObjectOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.bypass_governance_retention, None);
    app_ensure_eq!(input.header.if_match_last_modified_time, None);
    app_ensure_eq!(input.header.if_match_size, None);
    app_ensure_eq!(input.header.mfa, None);
    app_ensure_matches!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let (delete_marker, version_id) = if bucket.versioning.is_some() {
        let version = match input.query.version_id {
            Some(version_id) => VersionQuery::find(db.as_ref(), version_id).await?,
            None => ObjectQuery::find_version(db.as_ref(), bucket.id, &input.path.key)
                .await?
                .map(|(_, version)| version),
        };

        if let Some(version) = &version {
            VersionMutation::update_deleted_at(db.as_ref(), version.id).await?;
        }

        (
            version.as_ref().map(|version| version.deleted_at.is_none()),
            version.map(|version| version.id),
        )
    } else {
        if input.query.version_id.is_some() {
            Err(AppError::InternalError)?
        };

        ObjectMutation::delete(db.as_ref(), bucket.id, &input.path.key).await?;

        (None, None)
    };

    Ok(DeleteObjectOutput::builder()
        .header(
            DeleteObjectOutputHeader::builder()
                .maybe_delete_marker(delete_marker)
                .maybe_version_id(version_id)
                .build(),
        )
        .build())
}

#[instrument(skip(db_conn, db), ret)]
async fn get_object(
    State(db_conn): State<DbConn>,
    Extension(db): Extension<DbTxn>,
    input: GetObjectInput,
) -> AppResult<GetObjectOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.if_modified_since, None);
    app_ensure_eq!(input.header.if_none_match, None);
    app_ensure_eq!(input.header.if_unmodified_since, None);
    app_ensure_matches!(
        input.header.range.as_ref().map(|range| range.ranges.len()),
        None | Some(1) // todo multipart/byteranges
    );
    app_ensure_matches!(input.header.checksum_mode, None);
    app_ensure_matches!(input.header.request_payer, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key_md5, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version = match input.query.version_id {
        Some(version_id) => VersionQuery::find(db.as_ref(), version_id).await?,
        None => ObjectQuery::find_version(db.as_ref(), bucket.id, &input.path.key)
            .await?
            .map(|(_, version)| version),
    }
    .ok_or(AppError::NoSuchKey)?;
    if version.deleted_at.is_some() {
        return Ok(GetObjectOutput::builder()
            .status(if input.query.version_id.is_some() {
                StatusCode::METHOD_NOT_ALLOWED
            } else {
                StatusCode::NOT_FOUND
            })
            .header(
                GetObjectOutputHeader::builder()
                    .maybe_last_modified(input.query.version_id.map(|_version_id| {
                        SystemTime::from(version.updated_at.unwrap_or(version.created_at))
                    }))
                    .delete_marker(true)
                    .build(),
            )
            .body(Body::empty())
            .build());
    }
    let (part_id, size, e_tag, last_modified) = match input.query.part_number {
        Some(part_number) => {
            #[cfg(not(feature = "part-number-with-range"))]
            if input.header.range.is_some() {
                Err(AppError::InternalError)?
            }

            let part = PartQuery::find(db.as_ref(), None, Some(version.id), part_number)
                .await?
                .ok_or(AppError::InvalidPart)?;

            (
                Some(part.id),
                part.size as u64,
                part.e_tag,
                part.updated_at.unwrap_or(part.created_at),
            )
        }
        None => (
            None,
            version.size as u64,
            version.e_tag,
            version.updated_at.unwrap_or(version.created_at),
        ),
    };
    let range = input
        .header
        .range
        .map(|ranges| {
            ranges
                .validate(size)
                .map(|mut ranges| ranges.pop().unwrap_or_else(|| unreachable!()))
                .map_err(|_err| AppError::InvalidRange)
        })
        .transpose()?;
    let body = match part_id {
        Some(part_id) => ChunkQuery::stream_by_part_id(db_conn, part_id, range.clone())
            .await
            .left_stream(),
        None => ChunkQuery::stream_by_version_id(db_conn, version.id, range.clone())
            .await
            .right_stream(),
    };

    Ok(GetObjectOutput::builder()
        .header(
            GetObjectOutputHeader::builder()
                .accept_ranges("bytes".to_owned())
                .maybe_cache_control(input.query.response_cache_control)
                .maybe_content_disposition(input.query.response_content_disposition)
                .maybe_content_encoding(input.query.response_content_encoding)
                .maybe_content_language(input.query.response_content_language)
                .content_length(
                    range
                        .as_ref()
                        .map(|range| range.end() - range.start() + 1)
                        .unwrap_or(size),
                )
                .maybe_content_range(range.map(|range| ContentRangeBytes {
                    first_byte: *range.start(),
                    last_byte: *range.end(),
                    complete_length: size,
                }))
                .maybe_content_type(input.query.response_content_type)
                .e_tag(e_tag)
                .maybe_expires(input.query.response_expires)
                .last_modified(SystemTime::from(last_modified))
                // todo mp_parts_count
                .maybe_version_id(bucket.versioning.unwrap_or_default().then_some(version.id))
                .build(),
        )
        .body(Body::from_stream(body))
        .build())
}

#[instrument(skip(db), ret)]
async fn head_object(
    Extension(db): Extension<DbTxn>,
    input: HeadObjectInput,
) -> AppResult<HeadObjectOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.if_modified_since, None);
    app_ensure_eq!(input.header.if_none_match, None);
    app_ensure_eq!(input.header.if_unmodified_since, None);
    app_ensure_matches!(
        input.header.range.as_ref().map(|range| range.ranges.len()),
        None | Some(1) // todo multipart/byteranges
    );
    app_ensure_matches!(input.header.checksum_mode, None);
    app_ensure_matches!(input.header.request_payer, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key_md5, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version = match input.query.version_id {
        Some(version_id) => VersionQuery::find(db.as_ref(), version_id).await?,
        None => ObjectQuery::find_version(db.as_ref(), bucket.id, &input.path.key)
            .await?
            .map(|(_, version)| version),
    }
    .ok_or(AppError::NoSuchKey)?;
    if version.deleted_at.is_some() {
        return Ok(HeadObjectOutput::builder()
            .status(if input.query.version_id.is_some() {
                StatusCode::METHOD_NOT_ALLOWED
            } else {
                StatusCode::NOT_FOUND
            })
            .header(
                HeadObjectOutputHeader::builder()
                    .maybe_last_modified(input.query.version_id.map(|_version_id| {
                        SystemTime::from(version.updated_at.unwrap_or(version.created_at))
                    }))
                    .delete_marker(true)
                    .build(),
            )
            .build());
    }
    let (size, e_tag, last_modified) = match input.query.part_number {
        Some(part_number) => {
            #[cfg(not(feature = "ranged-part"))]
            if input.header.range.is_some() {
                Err(AppError::InternalError)?
            }

            let part = PartQuery::find(db.as_ref(), None, Some(version.id), part_number)
                .await?
                .ok_or(AppError::InvalidPart)?;

            (
                part.size as u64,
                part.e_tag,
                part.updated_at.unwrap_or(part.created_at),
            )
        }
        None => (
            version.size as u64,
            version.e_tag,
            version.updated_at.unwrap_or(version.created_at),
        ),
    };
    let range = input
        .header
        .range
        .map(|ranges| {
            ranges
                .validate(size)
                .map(|mut ranges| ranges.pop().unwrap_or_else(|| unreachable!()))
                .map_err(|_err| AppError::InvalidRange)
        })
        .transpose()?;

    Ok(HeadObjectOutput::builder()
        .header(
            HeadObjectOutputHeader::builder()
                .accept_ranges("bytes".to_owned())
                .maybe_cache_control(input.query.response_cache_control)
                .maybe_content_disposition(input.query.response_content_disposition)
                .maybe_content_encoding(input.query.response_content_encoding)
                .maybe_content_language(input.query.response_content_language)
                .content_length(
                    range
                        .as_ref()
                        .map(|range| range.end() - range.start() + 1)
                        .unwrap_or(version.size as u64),
                )
                .maybe_content_range(range.map(|range| ContentRangeBytes {
                    first_byte: *range.start(),
                    last_byte: *range.end(),
                    complete_length: version.size as u64,
                }))
                .maybe_content_type(input.query.response_content_type)
                .e_tag(e_tag)
                .maybe_expires(input.query.response_expires)
                .last_modified(SystemTime::from(last_modified))
                // todo mp_parts_count
                .maybe_version_id(bucket.versioning.unwrap_or_default().then_some(version.id))
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn put_object(
    Extension(db): Extension<DbTxn>,
    input: PutObjectInput,
) -> AppResult<PutObjectOutput> {
    let owner = OwnerQuery::find(db.as_ref(), "minil").await?.unwrap();

    app_ensure_eq!(input.header.cache_control, None);
    app_ensure_eq!(input.header.content_disposition, None);
    app_ensure_eq!(input.header.content_encoding, None);
    app_ensure_eq!(input.header.content_language, None);
    app_ensure_eq!(input.header.expires, None);
    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.if_none_match, None);
    app_ensure_matches!(input.header.acl, None);
    app_ensure_eq!(input.header.grant_full_control, None);
    app_ensure_eq!(input.header.grant_read, None);
    app_ensure_eq!(input.header.grant_read_acp, None);
    app_ensure_eq!(input.header.grant_write_acp, None);
    app_ensure_matches!(input.header.object_lock_legal_hold, None);
    app_ensure_matches!(input.header.object_lock_mode, None);
    app_ensure_eq!(input.header.object_lock_retain_until_date, None);
    app_ensure_matches!(input.header.request_payer, None);
    app_ensure_matches!(input.header.sdk_checksum_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption, None);
    app_ensure_eq!(input.header.server_side_encryption_aws_kms_key_id, None);
    app_ensure_eq!(input.header.server_side_encryption_bucket_key_enabled, None);
    app_ensure_eq!(input.header.server_side_encryption_context, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key_md5, None);
    app_ensure_matches!(input.header.storage_class, None);
    app_ensure_eq!(input.header.tagging, None);
    app_ensure_eq!(input.header.website_redirect_location, None);
    app_ensure_matches!(input.header.write_offset_bytes, None | Some(0));

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version_id = if bucket.versioning.unwrap_or_default() {
        None
    } else {
        ObjectQuery::find(db.as_ref(), bucket.id, &input.path.key)
            .await?
            .map(|object| object.version_id)
    };

    let (_, version) = ObjectMutation::insert_version(
        db.as_ref(),
        bucket.id,
        input.path.key,
        version_id,
        input.header.content_type,
        StreamReader::new(
            input
                .body
                .into_data_stream()
                .map(|res| res.map_err(|err| io::Error::other(err.into_inner()))),
        ),
    )
    .await?;
    app_validate_digest!(input.header.content_md5, version.md5.clone());
    app_validate_digest!(input.header.checksum_crc32, version.crc32);
    app_validate_digest!(input.header.checksum_crc32c, version.crc32c);
    app_validate_digest!(input.header.checksum_crc64nvme, version.crc64nvme);
    app_validate_digest!(input.header.checksum_sha1, version.sha1);
    app_validate_digest!(input.header.checksum_sha256, version.sha256);

    Ok(PutObjectOutput::builder()
        .header(
            PutObjectOutputHeader::builder()
                .e_tag(version.e_tag)
                .maybe_version_id(
                    bucket
                        .versioning
                        .and_then(|versioning| versioning.then_some(version.id)),
                )
                .build(),
        )
        .build())
}
