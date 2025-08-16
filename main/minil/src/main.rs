mod database_transaction;
mod error;
mod macros;
mod state;
mod utils;

use std::collections::HashSet;
use std::convert;
use std::env;
use std::future;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;

use async_stream::__private::AsyncStream;
use async_stream::try_stream;
use axum::Extension;
use axum::Router;
use axum::ServiceExt;
use axum::body::Body;
use axum::extract::Request;
use axum::extract::State;
use axum::handler::Handler;
use axum::http::HeaderName;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;
use axum_s3::operation::*;
use axum_s3::utils::CommonExtInput;
use futures::StreamExt;
use futures::TryStreamExt;
use http_content_range::ContentRangeBytes;
use http_digest::DigestMd5;
use indexmap::IndexSet;
use md5::Digest;
use md5::Md5;
use mime::Mime;
use minil_config::AppConfig;
use minil_migration::Migrator;
use minil_migration::MigratorTrait;
use minil_service::prelude::*;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DbConn;
use sea_orm::TransactionTrait;
use serde_s3::operation::*;
use serde_s3::types::Bucket;
use serde_s3::types::BucketLocationConstraint;
use serde_s3::types::BucketVersioningStatus;
use serde_s3::types::CommonPrefix;
use serde_s3::types::DeleteMarkerEntry;
use serde_s3::types::EncodingType;
use serde_s3::types::Initiator;
use serde_s3::types::MfaDeleteStatus;
use serde_s3::types::MultipartUpload;
use serde_s3::types::Object;
use serde_s3::types::ObjectVersion;
use serde_s3::types::Owner;
use serde_s3::types::Part;
use serde_s3::types::Tag;
use serde_s3::utils::DeleteMarkerOrVersion;
use tokio::net::TcpListener;
use tower::Layer;
use tower::ServiceBuilder;
use tower_http::BoxError;
use tower_http::ServiceBuilderExt;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::request_id::MakeRequestUuid;
use tower_http::set_header::SetRequestHeaderLayer;
use tracing::debug;
use tracing::info;
use tracing::instrument;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::prelude::*;
use uuid::Uuid;

use crate::database_transaction::DbTxn;
use crate::error::AppError;
use crate::error::AppErrorDiscriminants;
use crate::error::AppResult;
use crate::macros::app_define_handlers;
use crate::macros::app_ensure_eq;
use crate::macros::app_ensure_matches;
use crate::macros::app_validate_owner;
use crate::state::AppState;
use crate::utils::BodyExt;
use crate::utils::ServiceBuilderExt as _;

#[cfg(debug_assertions)]
#[global_allocator]
static ALLOCATOR: cap::Cap<std::alloc::System> = cap::Cap::new(
    std::alloc::System,
    bytesize::ByteSize::mib(100).as_u64() as usize,
);

const NODE_NAME: &str = "     minil     \0";
const NODE_REGION: &str = "us-east-1";
const PROCESS_TIME: &str = "x-process-time";

const NODE_ID_HEADER: HeaderName = HeaderName::from_static("x-amz-id-2");
const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-amz-request-id");

#[tokio::main]
async fn main() {
    let config = dbg!(init_config());
    let _log_guard = init_trace(&config);
    let db = init_db(&config).await;

    let state = AppState::new(db);
    let node_id =
        Uuid::new_v8(NODE_NAME.as_bytes().try_into().expect("invalid node name")).to_string();
    let server = format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let middleware = ServiceBuilder::new()
        .trace_for_http()
        .decompression()
        // fixme .compression()
        .override_request_header(
            NODE_ID_HEADER,
            node_id.parse::<HeaderValue>().expect("invalid node id"),
        )
        .propagate_header(NODE_ID_HEADER)
        .set_request_id(REQUEST_ID_HEADER, MakeRequestUuid)
        .propagate_request_id(REQUEST_ID_HEADER)
        .insert_response_header_if_not_present(
            header::SERVER,
            server
                .parse::<HeaderValue>()
                .expect("invalid server header"),
        )
        .middleware_fn(set_process_time)
        .middleware_fn(handle_app_err)
        .middleware_fn(validate_content_md5)
        .middleware_fn_with_state(state.clone(), manage_db_txn);

    let content_type_value = "application/xml"
        .parse::<HeaderValue>()
        .expect("invalid content type header");
    let if_not_present_content_type_layer =
        SetRequestHeaderLayer::if_not_present(header::CONTENT_TYPE, content_type_value.clone());
    let override_content_type_layer =
        SetRequestHeaderLayer::overriding(header::CONTENT_TYPE, content_type_value);

    let put_bucket_tagging_handler =
        put_bucket_tagging.layer(if_not_present_content_type_layer.clone());
    let put_object_tagging_handler = put_object_tagging.layer(if_not_present_content_type_layer);
    let complete_multipart_upload_handler =
        complete_multipart_upload.layer(override_content_type_layer);

    let router = Router::new();
    let router = app_define_handlers!(router {
        get("/") => list_buckets,

        delete("/{Bucket}") => {
            query("tagging", "") => delete_bucket_tagging,
            _ => delete_bucket,
        },
        get("/{Bucket}") => {
            query("list-type", "2") => list_objects_v2,
            query("location", "") => get_bucket_location,
            query("versioning", "") => get_bucket_versioning,
            query("versions", "") => list_object_versions,
            query("tagging", "") => get_bucket_tagging,
            query("uploads", "") => list_multipart_uploads,
            _ => list_objects,
        },
        head("/{Bucket}") => head_bucket,
        put("/{Bucket}") => {
            query("tagging", "") => put_bucket_tagging_handler,
            query("versioning", "") => put_bucket_versioning,
            _ => create_bucket,
        },

        delete("/{Bucket}/{*Key}") => {
            query("tagging", "") => delete_object_tagging,
            query("uploadId") => abort_multipart_upload,
            _ => delete_object
        },
        get("/{Bucket}/{*Key}") => {
            query("tagging", "") => get_object_tagging,
            query("uploadId") => list_parts,
            _ => get_object
        },
        head("/{Bucket}/{*Key}") => head_object,
        post("/{Bucket}/{*Key}") => {
            query("uploads", "") => create_multipart_upload,
            query("uploadId") => complete_multipart_upload_handler,
            _ => async || AppError::InternalError, // todo
        },
        put("/{Bucket}/{*Key}") => {
            query("tagging", "") => put_object_tagging_handler,
            query("uploadId") => upload_part,
            _ => put_object
        },

        // fixme "/" => get {
        //     header(r"foo?") => contains_fo_or_foo,
        //     scheme("http") & host("localhost") => debug_handler,
        //     scheme("http") ^ !host("localhost") => panic_handler,
        //     custom_handler => response_is_ok,
        // },
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
    info!("tcp listening on {addr}");
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
                debug!(%err, "DotEnvyError");
            }
        },
    }

    AppConfig::try_new().expect("failed to load config")
}

fn init_trace(config: &AppConfig) -> WorkerGuard {
    let (writer, guard) = config.log.stream.to_writer();
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

    guard
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
        () = ctrl_c => {},
        () = terminate => {},
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

async fn handle_app_err(common: CommonExtInput, request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    response
        .extensions_mut()
        .remove::<AppErrorDiscriminants>()
        .map_or(response, |err| err.into_response(common))
}

//noinspection RsBorrowChecker
async fn validate_content_md5(request: Request, next: Next) -> AppResult<Response> {
    let request = if let Some(content_md5) = request
        .headers()
        .get(HeaderName::from_static("content-md5"))
    {
        let content_md5 = content_md5
            .to_str()
            .map_err(|_| AppError::InvalidDigest)?
            .parse::<DigestMd5>()
            .map_err(|_| AppError::InvalidDigest)?;

        let (parts, body) = request.into_parts();

        let stream: AsyncStream<Result<_, BoxError>, _> = try_stream! {
            let mut md5 = Md5::new();
            let mut stream = pin!(body.into_data_stream());

            while let Some(chunk) = stream.try_next().await? {
                md5.update(&chunk);
                yield chunk
            }

            if content_md5.as_bytes() != md5.finalize().as_slice() {
                Err(AppError::BadDigest)?;
            }
        };

        let body = Body::from_stream(stream);
        Request::from_parts(parts, body)
    } else {
        request
    };

    Ok(next.run(request).await)
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
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    let limit = input.query.max_buckets + 1;
    let mut buckets = BucketQuery::find_many(
        &*db,
        owner.id,
        input.query.prefix.as_deref(),
        input.query.continuation_token.as_deref(),
        Some(limit.into()),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let continuation_token = (buckets.len() == limit as usize).then(|| {
        buckets.pop();
        buckets.last().unwrap().name.clone()
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

#[instrument(skip(db), ret)] // todo silence err
async fn delete_bucket_tagging(
    Extension(db): Extension<DbTxn>,
    input: DeleteBucketTaggingInput,
) -> AppResult<DeleteBucketTaggingOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    TagSetMutation::delete(&*db, Some(bucket.id), None, None)
        .await?
        .ok_or(AppError::NoSuchTagSet)?;

    Ok(DeleteBucketTaggingOutput::builder().build())
}

#[instrument(skip(db), ret)]
async fn delete_bucket(
    Extension(db): Extension<DbTxn>,
    input: DeleteBucketInput,
) -> AppResult<DeleteBucketOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    BucketMutation::delete(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    Ok(DeleteBucketOutput::builder().build())
}

#[instrument(skip(db), ret)]
async fn list_objects_v2(
    Extension(db): Extension<DbTxn>,
    input: ListObjectsV2Input,
) -> AppResult<ListObjectsV2Output> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_matches!(input.header.optional_object_attributes, None);
    app_ensure_eq!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let limit = input.query.max_keys + 1;
    let mut objects_versions = ObjectQuery::find_many_both_latest_version(
        &*db,
        bucket.id,
        input.query.prefix.as_deref(),
        input
            .query
            .continuation_token
            .as_deref()
            .or(input.query.start_after.as_deref()),
        Some(limit.into()),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let next_continuation_token = (objects_versions.len() == limit as usize).then(|| {
        objects_versions.pop();
        objects_versions.last().unwrap().0.key.clone()
    });
    let mut common_prefixes = IndexSet::new();
    if let Some(delimiter) = &input.query.delimiter {
        let prefix_len = input
            .query
            .prefix
            .as_deref()
            .map(str::len)
            .unwrap_or_default();
        let offset = prefix_len + delimiter.len();
        objects_versions.retain(|(object, _)| {
            if let Some(delimiter_index) = &object.key[prefix_len..].find(delimiter) {
                common_prefixes.insert(object.key[..offset + delimiter_index].to_owned());

                false
            } else {
                true
            }
        });
    }
    let key_count = objects_versions.len() + common_prefixes.len();
    let encode = if let Some(encoding_type) = &input.query.encoding_type {
        match encoding_type {
            EncodingType::Url => |string: String| urlencoding::encode(&string).into_owned(),
        }
    } else {
        convert::identity
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
                    objects_versions
                        .into_iter()
                        .map(|(object, version)| {
                            Object::builder()
                                .e_tag(version.e_tag())
                                .key(encode(object.key))
                                .last_modified(version.last_modified())
                                .maybe_owner(input.query.fetch_owner.unwrap_or_default().then(
                                    || {
                                        Owner::builder()
                                            .display_name(owner.name.clone())
                                            .id(owner.id)
                                            .build()
                                    },
                                ))
                                .size(version.size())
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
async fn get_bucket_location(
    Extension(db): Extension<DbTxn>,
    input: GetBucketLocationInput,
) -> AppResult<GetBucketLocationOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    BucketQuery::find(&*db, owner.id, &input.path.bucket)
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
async fn get_bucket_versioning(
    Extension(db): Extension<DbTxn>,
    input: GetBucketVersioningInput,
) -> AppResult<GetBucketVersioningOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
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
async fn list_object_versions(
    Extension(db): Extension<DbTxn>,
    input: ListObjectVersionsInput,
) -> AppResult<ListObjectVersionsOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_matches!(input.header.optional_object_attributes, None);
    app_ensure_eq!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let limit = input.query.max_keys + 1;
    let mut versions_objects = VersionQuery::find_many_both_object(
        &*db,
        bucket.id,
        input.query.prefix.as_deref(),
        input.query.key_marker.as_deref(),
        input.query.version_id_marker.as_deref(),
        Some(limit.into()),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let (next_key_marker, next_version_id_marker) = (versions_objects.len() == limit as usize)
        .then(|| {
            let (version, object) = versions_objects.pop().unwrap();
            (object.key, version.id)
        })
        .unzip();
    let mut common_prefixes = IndexSet::new();
    if let Some(delimiter) = &input.query.delimiter {
        let prefix_len = input
            .query
            .prefix
            .as_deref()
            .map(str::len)
            .unwrap_or_default();
        let offset = prefix_len + delimiter.len();
        versions_objects.retain(|(_, object)| {
            if let Some(delimiter_index) = &object.key[prefix_len..].find(delimiter) {
                common_prefixes.insert(object.key[..offset + delimiter_index].to_owned());

                false
            } else {
                true
            }
        });
    }
    let encode = if let Some(encoding_type) = &input.query.encoding_type {
        match encoding_type {
            EncodingType::Url => |string: String| urlencoding::encode(&string).into_owned(),
        }
    } else {
        convert::identity
    };
    let mut delete_marker = vec![];
    let mut version = vec![];
    let mut delete_marker_or_version = versions_objects.into_iter().map(|(version, object)| {
        let is_latest = object.version_id == version.id;
        let key = encode(object.key);
        let last_modified = version.last_modified();
        let owner = Owner::builder()
            .display_name(owner.name.clone())
            .id(owner.id)
            .build();
        let version_id = version.id();

        if version.parts_count.is_none() {
            DeleteMarkerEntry::builder()
                .is_latest(is_latest)
                .key(key)
                .last_modified(version.last_modified())
                .owner(owner)
                .version_id(version_id)
                .build()
                .into()
        } else {
            ObjectVersion::builder()
                .e_tag(version.e_tag())
                .is_latest(is_latest)
                .key(key)
                .last_modified(last_modified)
                .owner(owner)
                .size(version.size())
                .version_id(version_id)
                .build()
                .into()
        }
    });
    if cfg!(feature = "separate-version") {
        for delete_marker_or_version in delete_marker_or_version.by_ref() {
            match delete_marker_or_version {
                DeleteMarkerOrVersion::DeleteMarker(delete_marker_entry) => {
                    delete_marker.push(delete_marker_entry);
                }
                DeleteMarkerOrVersion::Version(object_version) => {
                    version.push(object_version);
                }
            }
        }
    }

    Ok(ListObjectVersionsOutput::builder()
        .header(ListObjectVersionsOutputHeader::builder().build())
        .body(
            ListObjectVersionsOutputBody::builder()
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
                .delete_marker(delete_marker)
                .maybe_delimiter(input.query.delimiter.map(encode))
                .maybe_encoding_type(input.query.encoding_type)
                .is_truncated(next_version_id_marker.is_some())
                .key_marker(input.query.key_marker.map(encode).unwrap_or_default())
                .max_keys(input.query.max_keys)
                .name(bucket.name)
                .maybe_next_key_marker(next_key_marker.map(encode))
                .maybe_next_version_id_marker(next_version_id_marker)
                .prefix(input.query.prefix.map(encode).unwrap_or_default())
                .version(version)
                .version_id_marker(input.query.version_id_marker.unwrap_or_default())
                .delete_marker_or_version(delete_marker_or_version.collect())
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn get_bucket_tagging(
    Extension(db): Extension<DbTxn>,
    input: GetBucketTaggingInput,
) -> AppResult<GetBucketTaggingOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let tag_set = BucketQuery::find_also_tag_set(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?
        .1
        .ok_or(AppError::NoSuchTagSet)?;
    let tag_set = TagQuery::find_many(&*db, tag_set.id)
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    Ok(GetBucketTaggingOutput::builder()
        .body(
            GetBucketTaggingOutputBody::builder()
                .tag_set(
                    tag_set
                        .into_iter()
                        .map(|tag| Tag::builder().key(tag.key).value(tag.value).build())
                        .collect::<Vec<_>>(),
                )
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn list_multipart_uploads(
    Extension(db): Extension<DbTxn>,
    input: ListMultipartUploadsInput,
) -> AppResult<ListMultipartUploadsOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let limit = input.query.max_uploads + 1;
    let mut uploads = UploadQuery::find_many(
        &*db,
        bucket.id,
        input.query.prefix.as_deref(),
        input.query.key_marker.as_deref(),
        input.query.upload_id_marker.as_deref(),
        Some(limit.into()),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let (next_key_marker, next_upload_id_marker) = (uploads.len() == limit as usize)
        .then(|| {
            let upload = uploads.pop().unwrap();
            (upload.key, upload.id)
        })
        .unzip();
    let mut common_prefixes = IndexSet::new();
    if let Some(delimiter) = &input.query.delimiter {
        let prefix_len = input
            .query
            .prefix
            .as_deref()
            .map(str::len)
            .unwrap_or_default();
        let offset = prefix_len + delimiter.len();
        uploads.retain(|upload| {
            if let Some(delimiter_index) = &upload.key[prefix_len..].find(delimiter) {
                common_prefixes.insert(upload.key[..offset + delimiter_index].to_owned());

                false
            } else {
                true
            }
        });
    }
    let encode = if let Some(encoding_type) = &input.query.encoding_type {
        match encoding_type {
            EncodingType::Url => |string: String| urlencoding::encode(&string).into_owned(),
        }
    } else {
        convert::identity
    };

    Ok(ListMultipartUploadsOutput::builder()
        .header(ListMultipartUploadsOutputHeader::builder().build())
        .body(
            ListMultipartUploadsOutputBody::builder()
                .bucket(bucket.name)
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
                .maybe_delimiter(input.query.delimiter.map(encode))
                .maybe_encoding_type(input.query.encoding_type)
                .is_truncated(next_upload_id_marker.is_some())
                .key_marker(input.query.key_marker.map(encode).unwrap_or_default())
                .max_uploads(input.query.max_uploads)
                .next_key_marker(next_key_marker.map(encode).unwrap_or_default())
                .next_upload_id_marker(
                    next_upload_id_marker
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default(),
                )
                .maybe_prefix(input.query.prefix.map(encode))
                .upload(
                    uploads
                        .into_iter()
                        .map(|upload| {
                            MultipartUpload::builder()
                                .upload_id(upload.id)
                                .key(upload.key)
                                .initiated(upload.created_at)
                                .owner(
                                    Owner::builder()
                                        .display_name(owner.name.clone())
                                        .id(owner.id)
                                        .build(),
                                )
                                .initiator(
                                    Initiator::builder()
                                        .id(owner.id)
                                        .display_name(owner.name.clone())
                                        .build(),
                                )
                                .build()
                        })
                        .collect(),
                )
                .upload_id_marker(input.query.upload_id_marker.unwrap_or_default())
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn list_objects(
    Extension(db): Extension<DbTxn>,
    input: ListObjectsInput,
) -> AppResult<ListObjectsOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_matches!(input.header.optional_object_attributes, None);
    app_ensure_eq!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let limit = input.query.max_keys + 1;
    let mut objects_versions = ObjectQuery::find_many_both_latest_version(
        &*db,
        bucket.id,
        input.query.prefix.as_deref(),
        input.query.marker.as_deref(),
        Some(limit.into()),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let next_marker = (objects_versions.len() == limit as usize).then(|| {
        objects_versions.pop();
        objects_versions.last().unwrap().0.key.clone()
    });
    let mut common_prefixes = IndexSet::new();
    if let Some(delimiter) = &input.query.delimiter {
        let prefix_len = input
            .query
            .prefix
            .as_deref()
            .map(str::len)
            .unwrap_or_default();
        let offset = prefix_len + delimiter.len();
        objects_versions.retain(|(object, _)| {
            if let Some(delimiter_index) = &object.key[prefix_len..].find(delimiter) {
                common_prefixes.insert(object.key[..offset + delimiter_index].to_owned());

                false
            } else {
                true
            }
        });
    }
    let encode = if let Some(encoding_type) = &input.query.encoding_type {
        match encoding_type {
            EncodingType::Url => |string: String| urlencoding::encode(&string).into_owned(),
        }
    } else {
        convert::identity
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
                    objects_versions
                        .into_iter()
                        .map(|(object, version)| {
                            Object::builder()
                                .e_tag(version.e_tag())
                                .key(encode(object.key))
                                .last_modified(version.last_modified())
                                .owner(
                                    Owner::builder()
                                        .display_name(owner.name.clone())
                                        .id(owner.id)
                                        .build(),
                                )
                                .size(version.size())
                                .build()
                        })
                        .collect(),
                )
                .maybe_delimiter(input.query.delimiter.clone().map(encode))
                .maybe_encoding_type(input.query.encoding_type)
                .is_truncated(next_marker.is_some())
                .marker(input.query.marker.map(encode).unwrap_or_default())
                .max_keys(input.query.max_keys)
                .name(bucket.name)
                .maybe_next_marker(input.query.delimiter.and_then(|_| next_marker.map(encode)))
                .prefix(input.query.prefix.map(encode).unwrap_or_default())
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn head_bucket(
    Extension(db): Extension<DbTxn>,
    input: HeadBucketInput,
) -> AppResult<HeadBucketOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    Ok(HeadBucketOutput::builder()
        .header(
            HeadBucketOutputHeader::builder()
                .bucket_region(NODE_REGION.to_owned())
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn put_bucket_tagging(
    Extension(db): Extension<DbTxn>,
    input: PutBucketTaggingInput,
) -> AppResult<PutBucketTaggingOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.sdk_checksum_algorithm, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    if input.body.tag_set.tag.len() > 50 {
        Err(AppError::InvalidTag)?;
    }
    let mut keys = HashSet::new();
    if !input
        .body
        .tag_set
        .tag
        .iter()
        .all(|tag| keys.insert(&tag.key))
    {
        Err(AppError::InvalidTag)?;
    }
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    TagSetMutation::upsert_with_tag(
        &*db,
        Some(bucket.id),
        None,
        None,
        input
            .body
            .tag_set
            .tag
            .into_iter()
            .map(|tag| (tag.key, tag.value)),
    )
    .await?;

    Ok(PutBucketTaggingOutput::builder().build())
}

#[instrument(skip(db), ret)]
async fn put_bucket_versioning(
    Extension(db): Extension<DbTxn>,
    input: PutBucketVersioningInput,
) -> AppResult<PutBucketVersioningOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.mfa, None);
    app_ensure_eq!(input.header.sdk_checksum_algorithm, None);
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
    BucketMutation::update_versioning(&*db, owner.id, &input.path.bucket, mfa_delete, status)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    Ok(PutBucketVersioningOutput::builder().build())
}

#[instrument(skip(db), ret)]
async fn create_bucket(
    Extension(db): Extension<DbTxn>,
    input: CreateBucketInput,
) -> AppResult<CreateBucketOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_matches!(input.header.acl, None);
    app_ensure_matches!(input.header.bucket_object_lock_enabled, None | Some(false));
    app_ensure_eq!(input.header.grant_full_control, None);
    app_ensure_eq!(input.header.grant_read, None);
    app_ensure_eq!(input.header.grant_read_acp, None);
    app_ensure_eq!(input.header.grant_write, None);
    app_ensure_eq!(input.header.grant_write_acp, None);
    app_ensure_matches!(input.header.object_ownership, None);
    app_ensure_matches!(
        input.body.as_ref().and_then(|body| body.bucket.as_ref()),
        None
    );
    app_ensure_matches!(
        input.body.as_ref().and_then(|body| body.location.as_ref()),
        None
    );
    app_ensure_matches!(
        input
            .body
            .as_ref()
            .and_then(|body| body.location_constraint.as_ref()),
        None
    );

    let bucket = BucketMutation::insert(&*db, owner.id, input.path.bucket)
        .await?
        .ok_or(AppError::BucketAlreadyOwnedByYou)?;
    if let Some(tags) = input.body.and_then(|body| body.tags) {
        if tags.tag.len() > 50 {
            Err(AppError::InvalidTag)?;
        }
        let mut keys = HashSet::new();
        if !tags.tag.iter().all(|tag| keys.insert(&tag.key)) {
            Err(AppError::InvalidTag)?;
        }
        TagSetMutation::upsert_with_tag(
            &*db,
            Some(bucket.id),
            None,
            None,
            tags.tag.into_iter().map(|tag| (tag.key, tag.value)),
        )
        .await?;
    }

    Ok(CreateBucketOutput::builder()
        .header(
            CreateBucketOutputHeader::builder()
                .location(format!("/{}", bucket.name))
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn delete_object_tagging(
    Extension(db): Extension<DbTxn>,
    input: DeleteObjectTaggingInput,
) -> AppResult<DeleteObjectTaggingOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version = match input.query.version_id {
        Some(version_id) => {
            if version_id.is_nil() {
                ObjectQuery::find_also_null_version(&*db, bucket.id, &input.path.key).await?
            } else {
                ObjectQuery::find_also_version(&*db, bucket.id, &input.path.key, version_id).await?
            }
        }
        None => ObjectQuery::find_both_latest_version(&*db, bucket.id, &input.path.key)
            .await?
            .map(|(object, version)| (object, Some(version))),
    }
    .ok_or(AppError::NoSuchKey)?
    .1
    .ok_or(AppError::NoSuchVersion)?;
    TagSetMutation::delete(&*db, None, None, Some(version.id))
        .await?
        .ok_or(AppError::NoSuchTagSet)?;

    Ok(DeleteObjectTaggingOutput::builder()
        .header(
            DeleteObjectTaggingOutputHeader::builder()
                .maybe_version_id(bucket.versioning.map(|_| version.id()))
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn abort_multipart_upload(
    Extension(db): Extension<DbTxn>,
    input: AbortMultipartUploadInput,
) -> AppResult<AbortMultipartUploadOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match_initiated_time, None);
    app_ensure_eq!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    UploadMutation::delete(&*db, input.query.upload_id, bucket.id, &input.path.key)
        .await?
        .ok_or(AppError::NoSuchUpload)?;

    Ok(AbortMultipartUploadOutput::builder()
        .header(AbortMultipartUploadOutputHeader::builder().build())
        .build())
}

#[instrument(skip(db), ret)] // todo silence err
async fn delete_object(
    Extension(db): Extension<DbTxn>,
    input: DeleteObjectInput,
) -> AppResult<DeleteObjectOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.bypass_governance_retention, None);
    app_ensure_eq!(input.header.if_match_last_modified_time, None);
    app_ensure_eq!(input.header.if_match_size, None);
    app_ensure_eq!(input.header.mfa, None);
    app_ensure_eq!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let (delete_marker, version_id) = match (input.query.version_id, bucket.versioning) {
        (Some(version_id), Some(_)) => {
            let delete_marker = ObjectMutation::delete_also_version_nullable(
                &*db,
                bucket.id,
                &input.path.key,
                version_id,
            )
            .await?
            .ok_or(AppError::NoSuchKey)?
            .1
            .ok_or(AppError::NoSuchVersion)?
            .parts_count
            .is_none();

            (delete_marker, None)
        }
        (None, Some(versioning)) => {
            let version = if cfg!(feature = "create-delete") {
                ObjectMutation::upsert_also_delete_marker(
                    &*db,
                    bucket.id,
                    input.path.key,
                    versioning,
                )
                .await?
                .1
            } else {
                ObjectMutation::update_also_delete_marker(
                    &*db,
                    bucket.id,
                    &input.path.key,
                    versioning,
                )
                .await?
                .ok_or(AppError::NoSuchKey)?
                .1
            };

            (true, Some(version.id()))
        }
        (version_id, None) => {
            let object = ObjectMutation::delete(&*db, bucket.id, &input.path.key)
                .await?
                .ok_or(AppError::NoSuchKey)?;

            if let Some(version_id) = version_id {
                if !version_id.is_nil() && version_id != object.version_id {
                    Err(AppError::NoSuchVersion)?;
                }
            }

            (false, None)
        }
    };

    Ok(DeleteObjectOutput::builder()
        .header(
            DeleteObjectOutputHeader::builder()
                .delete_marker(delete_marker)
                .maybe_version_id(version_id)
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn get_object_tagging(
    Extension(db): Extension<DbTxn>,
    input: GetObjectTaggingInput,
) -> AppResult<GetObjectTaggingOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version = match input.query.version_id {
        Some(version_id) => {
            if version_id.is_nil() {
                ObjectQuery::find_also_null_version(&*db, bucket.id, &input.path.key).await?
            } else {
                ObjectQuery::find_also_version(&*db, bucket.id, &input.path.key, version_id).await?
            }
        }
        None => ObjectQuery::find_both_latest_version(&*db, bucket.id, &input.path.key)
            .await?
            .map(|(object, version)| (object, Some(version))),
    }
    .ok_or(AppError::NoSuchKey)?
    .1
    .ok_or(AppError::NoSuchVersion)?;
    let tag_set = TagSetQuery::find(&*db, None, None, Some(version.id))
        .await?
        .ok_or(AppError::NoSuchTagSet)?;
    let tags = TagQuery::find_many(&*db, tag_set.id)
        .await?
        .try_collect::<Vec<_>>()
        .await?;

    Ok(GetObjectTaggingOutput::builder()
        .header(
            GetObjectTaggingOutputHeader::builder()
                .maybe_version_id(bucket.versioning.map(|_| version.id()))
                .build(),
        )
        .body(
            GetObjectTaggingOutputBody::builder()
                .tag_set(
                    tags.into_iter()
                        .map(|tag| Tag::builder().key(tag.key).value(tag.value).build())
                        .collect::<Vec<_>>(),
                )
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn list_parts(
    Extension(db): Extension<DbTxn>,
    input: ListPartsInput,
) -> AppResult<ListPartsOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key_md5, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let (bucket, upload) = BucketQuery::find_also_upload(
        &*db,
        owner.id,
        &input.path.bucket,
        input.query.upload_id,
        &input.path.key,
    )
    .await?
    .ok_or(AppError::NoSuchBucket)?;
    let upload = upload.ok_or(AppError::NoSuchUpload)?;
    let limit = input.query.max_parts + 1;
    let mut parts = UploadPartQuery::find_many(
        &*db,
        upload.id,
        input.query.part_number_marker,
        Some(limit.into()),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let next_part_number_marker = (parts.len() == limit as usize).then(|| {
        parts.pop();
        parts.last().unwrap().number as u16
    });

    Ok(ListPartsOutput::builder()
        .header(ListPartsOutputHeader::builder().build())
        .body(
            ListPartsOutputBody::builder()
                .bucket(bucket.name)
                .initiator(
                    Initiator::builder()
                        .id(owner.id)
                        .display_name(owner.name.clone())
                        .build(),
                )
                .is_truncated(next_part_number_marker.is_some())
                .key(upload.key)
                .max_parts(input.query.max_parts)
                .maybe_next_part_number_marker(next_part_number_marker)
                .owner(
                    Owner::builder()
                        .display_name(owner.name)
                        .id(owner.id)
                        .build(),
                )
                .part(
                    parts
                        .into_iter()
                        .map(|part| {
                            Part::builder()
                                .part_number(part.number as u16)
                                .last_modified(part.last_modified())
                                .e_tag(part.e_tag())
                                .size(part.size as u64)
                                .build()
                        })
                        .collect(),
                )
                .maybe_part_number_marker(input.query.part_number_marker)
                .upload_id(upload.id)
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
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.if_modified_since, None);
    app_ensure_eq!(input.header.if_none_match, None);
    app_ensure_eq!(input.header.if_unmodified_since, None);
    app_ensure_matches!(
        input.header.range.as_ref().map(|range| range.ranges.len()),
        None | Some(1) // todo multipart/byteranges
    );
    app_ensure_matches!(input.header.checksum_mode, None);
    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key_md5, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version = match input.query.version_id {
        Some(version_id) => {
            if version_id.is_nil() {
                ObjectQuery::find_also_null_version(&*db, bucket.id, &input.path.key).await?
            } else {
                ObjectQuery::find_also_version(&*db, bucket.id, &input.path.key, version_id).await?
            }
        }
        None => ObjectQuery::find_both_latest_version(&*db, bucket.id, &input.path.key)
            .await?
            .map(|(object, version)| (object, Some(version))),
    }
    .ok_or(AppError::NoSuchKey)?
    .1
    .ok_or(AppError::NoSuchVersion)?;
    if version.parts_count.is_none() {
        return Ok(GetObjectOutput::builder()
            .status(if input.query.version_id.is_some() {
                StatusCode::METHOD_NOT_ALLOWED
            } else {
                StatusCode::NOT_FOUND
            })
            .header(
                GetObjectOutputHeader::builder()
                    .maybe_last_modified(
                        input
                            .query
                            .version_id
                            .map(|_| SystemTime::from(version.last_modified())),
                    )
                    .delete_marker(true)
                    .build(),
            )
            .body(Body::empty())
            .build());
    }
    let (part_id, size, e_tag, last_modified) = match input.query.part_number {
        Some(part_number) => {
            if cfg!(not(feature = "ranged-part")) && input.header.range.is_some() {
                Err(AppError::InternalError)?;
            }

            let part = VersionPartQuery::find(&*db, version.id, part_number)
                .await?
                .ok_or(AppError::InvalidPart)?;

            (
                Some(part.id),
                part.size(),
                part.e_tag(),
                part.last_modified(),
            )
        }
        None => (
            None,
            version.size(),
            version.e_tag(),
            version.last_modified(),
        ),
    };
    let range = input
        .header
        .range
        .map(|ranges| {
            ranges
                .validate(size)
                .map(|mut ranges| ranges.pop().unwrap())
                .map_err(|_| AppError::InvalidRange)
        })
        .transpose()?;
    let body = match part_id {
        Some(part_id) => ChunkQuery::find_many_ranged_part_data_by_version_part_id(
            db_conn,
            part_id,
            range.clone(),
        )
        .left_stream(),
        None => ChunkQuery::find_many_ranged_version_data_by_version_id(
            db_conn,
            version.id,
            range.clone(),
        )
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
                        .map_or(size, |range| range.end() - range.start() + 1),
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
                .maybe_mp_parts_count(version.mp_parts_count())
                .maybe_version_id(bucket.versioning.map(|_| version.id()))
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
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.if_modified_since, None);
    app_ensure_eq!(input.header.if_none_match, None);
    app_ensure_eq!(input.header.if_unmodified_since, None);
    app_ensure_matches!(
        input.header.range.as_ref().map(|range| range.ranges.len()),
        None | Some(1) // todo multipart/byteranges
    );
    app_ensure_matches!(input.header.checksum_mode, None);
    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key_md5, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version = match input.query.version_id {
        Some(version_id) => {
            if version_id.is_nil() {
                ObjectQuery::find_also_null_version(&*db, bucket.id, &input.path.key).await?
            } else {
                ObjectQuery::find_also_version(&*db, bucket.id, &input.path.key, version_id).await?
            }
        }
        None => ObjectQuery::find_both_latest_version(&*db, bucket.id, &input.path.key)
            .await?
            .map(|(object, version)| (object, Some(version))),
    }
    .ok_or(AppError::NoSuchKey)?
    .1
    .ok_or(AppError::NoSuchVersion)?;
    if version.parts_count.is_none() {
        return Ok(HeadObjectOutput::builder()
            .status(if input.query.version_id.is_some() {
                StatusCode::METHOD_NOT_ALLOWED
            } else {
                StatusCode::NOT_FOUND
            })
            .header(
                HeadObjectOutputHeader::builder()
                    .maybe_last_modified(
                        input
                            .query
                            .version_id
                            .map(|_| SystemTime::from(version.last_modified())),
                    )
                    .delete_marker(true)
                    .build(),
            )
            .build());
    }
    let (size, e_tag, last_modified) = match input.query.part_number {
        Some(part_number) => {
            if cfg!(not(feature = "ranged-part")) && input.header.range.is_some() {
                Err(AppError::InternalError)?;
            }

            let part = VersionPartQuery::find(&*db, version.id, part_number)
                .await?
                .ok_or(AppError::InvalidPart)?;

            (part.size(), part.e_tag(), part.last_modified())
        }
        None => (version.size(), version.e_tag(), version.last_modified()),
    };
    let range = input
        .header
        .range
        .map(|ranges| {
            ranges
                .validate(size)
                .map(|mut ranges| ranges.pop().unwrap())
                .map_err(|_| AppError::InvalidRange)
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
                        .map_or(size, |range| range.end() - range.start() + 1),
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
                .maybe_mp_parts_count(version.mp_parts_count())
                .maybe_version_id(bucket.versioning.map(|_| version.id()))
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn create_multipart_upload(
    Extension(db): Extension<DbTxn>,
    input: CreateMultipartUploadInput,
) -> AppResult<CreateMultipartUploadOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.cache_control, None);
    app_ensure_eq!(input.header.content_disposition, None);
    app_ensure_eq!(input.header.content_encoding, None);
    app_ensure_eq!(input.header.content_language, None);
    app_ensure_eq!(input.header.expires, None);
    app_ensure_matches!(input.header.acl, None);
    app_ensure_matches!(input.header.checksum_algorithm, None);
    app_ensure_matches!(input.header.checksum_type, None);
    app_ensure_eq!(input.header.grant_full_control, None);
    app_ensure_eq!(input.header.grant_read, None);
    app_ensure_eq!(input.header.grant_read_acp, None);
    app_ensure_eq!(input.header.grant_write_acp, None);
    app_ensure_matches!(input.header.object_lock_legal_hold, None);
    app_ensure_matches!(input.header.object_lock_mode, None);
    app_ensure_eq!(input.header.object_lock_retain_until_date, None);
    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_matches!(input.header.server_side_encryption, None);
    app_ensure_eq!(input.header.server_side_encryption_aws_kms_key_id, None);
    app_ensure_eq!(input.header.server_side_encryption_bucket_key_enabled, None);
    app_ensure_eq!(input.header.server_side_encryption_context, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key_md5, None);
    app_ensure_matches!(input.header.storage_class, None);
    app_ensure_eq!(input.header.website_redirect_location, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let upload = UploadMutation::insert(
        &*db,
        bucket.id,
        input.path.key.clone(),
        input.header.content_type.as_ref(),
    )
    .await?;
    if let Some(tagging) = input.header.tagging {
        if tagging.len() > 10 {
            Err(AppError::InvalidTag)?;
        }
        let mut keys = HashSet::new();
        if !tagging.iter().all(|tag| keys.insert(&tag.key)) {
            Err(AppError::InvalidTag)?;
        }
        TagSetMutation::upsert_with_tag(
            &*db,
            None,
            Some(upload.id),
            None,
            tagging.into_iter().map(|tag| (tag.key, tag.value)),
        )
        .await?;
    }

    Ok(CreateMultipartUploadOutput::builder()
        .header(CreateMultipartUploadOutputHeader::builder().build())
        .body(
            CreateMultipartUploadOutputBody::builder()
                .bucket(bucket.name)
                .key(upload.key)
                .upload_id(upload.id)
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn complete_multipart_upload(
    Extension(db): Extension<DbTxn>,
    input: CompleteMultipartUploadInput,
) -> AppResult<CompleteMultipartUploadOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.if_none_match, None);
    app_ensure_eq!(input.header.checksum_crc32, None);
    app_ensure_eq!(input.header.checksum_crc32c, None);
    app_ensure_eq!(input.header.checksum_crc64nvme, None);
    app_ensure_eq!(input.header.checksum_sha1, None);
    app_ensure_eq!(input.header.checksum_sha256, None);
    app_ensure_eq!(input.header.checksum_type, None);
    app_ensure_eq!(input.header.mp_object_size, None);
    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key_md5, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let mut numbers = HashSet::new();
    let filters = input
        .body
        .part
        .into_iter()
        .map(|part| {
            let number = part.part_number.ok_or(AppError::InternalError)?;
            if !numbers.insert(number) {
                Err(AppError::InternalError)?;
            }

            let md5 = part
                .e_tag
                .map(|e_tag| {
                    hex::decode(
                        e_tag
                            .strip_prefix('"')
                            .unwrap_or(&e_tag)
                            .strip_suffix('"')
                            .unwrap_or(&e_tag),
                    )
                    .map_err(|_| AppError::InternalError)
                    .and_then(|md5| {
                        if md5.len() == 16 {
                            Ok(md5)
                        } else {
                            Err(AppError::InternalError)
                        }
                    })
                })
                .transpose()?;

            Ok((number, None, None, None, None, None, md5))
        })
        .collect::<AppResult<Vec<_>>>()?;
    let (bucket, upload) = BucketQuery::find_also_upload(
        &*db,
        owner.id,
        &input.path.bucket,
        input.query.upload_id,
        &input.path.key,
    )
    .await?
    .ok_or(AppError::NoSuchBucket)?;
    let upload = upload.ok_or(AppError::NoSuchUpload)?;
    let parts = UploadPartQuery::find_many_filtered(&*db, upload.id, filters.into_iter())
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(AppError::InvalidPart)?;
    let (object, version) = ObjectMutation::upsert_also_version_from_parts(
        &*db,
        bucket.id,
        upload.key,
        bucket.versioning.unwrap_or_default(),
        upload
            .mime
            .map(|mime| mime.parse::<Mime>().unwrap())
            .as_ref(),
        parts.into_iter(),
    )
    .await?;
    UploadMutation::delete(&*db, upload.id, bucket.id, &object.key)
        .await?
        .ok_or(AppError::NoSuchUpload)?;

    Ok(CompleteMultipartUploadOutput::builder()
        .header(
            CompleteMultipartUploadOutputHeader::builder()
                .maybe_version_id(bucket.versioning.map(|_| version.id()))
                .build(),
        )
        .body(
            CompleteMultipartUploadOutputBody::builder()
                .bucket(bucket.name)
                .e_tag(version.e_tag())
                .key(object.key)
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn put_object_tagging(
    Extension(db): Extension<DbTxn>,
    input: PutObjectTaggingInput,
) -> AppResult<PutObjectTaggingOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_eq!(input.header.sdk_checksum_algorithm, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    if input.body.tag_set.tag.len() > 10 {
        Err(AppError::InvalidTag)?;
    }
    let mut keys = HashSet::new();
    if !input
        .body
        .tag_set
        .tag
        .iter()
        .all(|tag| keys.insert(&tag.key))
    {
        Err(AppError::InvalidTag)?;
    }
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let version = match input.query.version_id {
        Some(version_id) => {
            if version_id.is_nil() {
                ObjectQuery::find_also_null_version(&*db, bucket.id, &input.path.key).await?
            } else {
                ObjectQuery::find_also_version(&*db, bucket.id, &input.path.key, version_id).await?
            }
        }
        None => ObjectQuery::find_both_latest_version(&*db, bucket.id, &input.path.key)
            .await?
            .map(|(object, version)| (object, Some(version))),
    }
    .ok_or(AppError::NoSuchKey)?
    .1
    .ok_or(AppError::NoSuchVersion)?;
    if version.parts_count.is_none() {
        Err(AppError::InternalError)?;
    }
    TagSetMutation::upsert_with_tag(
        &*db,
        None,
        None,
        Some(version.id),
        input
            .body
            .tag_set
            .tag
            .into_iter()
            .map(|tag| (tag.key, tag.value)),
    )
    .await?;

    Ok(PutObjectTaggingOutput::builder()
        .header(
            PutObjectTaggingOutputHeader::builder()
                .maybe_version_id(bucket.versioning.map(|_| version.id()))
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn upload_part(
    Extension(db): Extension<DbTxn>,
    input: UploadPartInput,
) -> AppResult<UploadPartOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.checksum_crc32, None);
    app_ensure_eq!(input.header.checksum_crc32c, None);
    app_ensure_eq!(input.header.checksum_crc64nvme, None);
    app_ensure_eq!(input.header.checksum_sha1, None);
    app_ensure_eq!(input.header.checksum_sha256, None);
    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_eq!(input.header.sdk_checksum_algorithm, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key_md5, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let upload = BucketQuery::find_also_upload(
        &*db,
        owner.id,
        &input.path.bucket,
        input.query.upload_id,
        &input.path.key,
    )
    .await?
    .ok_or(AppError::NoSuchBucket)?
    .1
    .ok_or(AppError::NoSuchUpload)?;
    let part = UploadPartMutation::upsert_with_chunk(
        &*db,
        upload.id,
        input.query.part_number,
        input.body.into_data_read(),
    )
    .await?;

    Ok(UploadPartOutput::builder()
        .header(
            UploadPartOutputHeader::builder()
                .e_tag(part.e_tag())
                .build(),
        )
        .build())
}

#[instrument(skip(db), ret)]
async fn put_object(
    Extension(db): Extension<DbTxn>,
    input: PutObjectInput,
) -> AppResult<PutObjectOutput> {
    let owner = OwnerQuery::find(&*db, "minil").await?.unwrap();

    app_ensure_eq!(input.header.cache_control, None);
    app_ensure_eq!(input.header.content_disposition, None);
    app_ensure_eq!(input.header.content_encoding, None);
    app_ensure_eq!(input.header.content_language, None);
    app_ensure_eq!(input.header.expires, None);
    app_ensure_eq!(input.header.if_match, None);
    app_ensure_eq!(input.header.if_none_match, None);
    app_ensure_matches!(input.header.acl, None);
    app_ensure_eq!(input.header.checksum_crc32, None);
    app_ensure_eq!(input.header.checksum_crc32c, None);
    app_ensure_eq!(input.header.checksum_crc64nvme, None);
    app_ensure_eq!(input.header.checksum_sha1, None);
    app_ensure_eq!(input.header.checksum_sha256, None);
    app_ensure_eq!(input.header.grant_full_control, None);
    app_ensure_eq!(input.header.grant_read, None);
    app_ensure_eq!(input.header.grant_read_acp, None);
    app_ensure_eq!(input.header.grant_write_acp, None);
    app_ensure_matches!(input.header.object_lock_legal_hold, None);
    app_ensure_matches!(input.header.object_lock_mode, None);
    app_ensure_eq!(input.header.object_lock_retain_until_date, None);
    app_ensure_eq!(input.header.request_payer, None);
    app_ensure_eq!(input.header.sdk_checksum_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption, None);
    app_ensure_eq!(input.header.server_side_encryption_aws_kms_key_id, None);
    app_ensure_eq!(input.header.server_side_encryption_bucket_key_enabled, None);
    app_ensure_eq!(input.header.server_side_encryption_context, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key, None);
    app_ensure_eq!(input.header.server_side_encryption_customer_key_md5, None);
    app_ensure_matches!(input.header.storage_class, None);
    app_ensure_eq!(input.header.website_redirect_location, None);
    app_ensure_matches!(input.header.write_offset_bytes, None | Some(0));

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find(&*db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let (_, version) = ObjectMutation::upsert_also_version(
        &*db,
        bucket.id,
        input.path.key,
        bucket.versioning.unwrap_or_default(),
        input.header.content_type.as_ref(),
        input.body.into_data_read(),
    )
    .await?;
    if let Some(tagging) = input.header.tagging {
        if tagging.len() > 10 {
            Err(AppError::InvalidTag)?;
        }
        let mut keys = HashSet::new();
        if !tagging.iter().all(|tag| keys.insert(&tag.key)) {
            Err(AppError::InvalidTag)?;
        }
        TagSetMutation::upsert_with_tag(
            &*db,
            None,
            None,
            Some(version.id),
            tagging.into_iter().map(|tag| (tag.key, tag.value)),
        )
        .await?;
    }

    Ok(PutObjectOutput::builder()
        .header(
            PutObjectOutputHeader::builder()
                .e_tag(version.e_tag())
                .maybe_version_id(bucket.versioning.map(|_| version.id()))
                .build(),
        )
        .build())
}
