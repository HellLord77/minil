mod database_transaction;
mod error;
mod macros;
mod make_request_id;
mod service_builder_ext;
mod state;

use std::env;
use std::future;
use std::io;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::sync::Arc;
use std::time::Instant;

use axum::Extension;
use axum::Router;
use axum::ServiceExt;
use axum::extract::Request;
use axum::extract::State;
use axum::http::HeaderName;
use axum::http::HeaderValue;
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;
use axum_s3::operation::CreateBucketInput;
use axum_s3::operation::CreateBucketOutput;
use axum_s3::operation::DeleteBucketInput;
use axum_s3::operation::DeleteBucketOutput;
use axum_s3::operation::GetBucketLocationInput;
use axum_s3::operation::GetBucketLocationOutput;
use axum_s3::operation::GetBucketVersioningInput;
use axum_s3::operation::GetBucketVersioningOutput;
use axum_s3::operation::HeadBucketInput;
use axum_s3::operation::HeadBucketOutput;
use axum_s3::operation::ListBucketsInput;
use axum_s3::operation::ListBucketsOutput;
use axum_s3::operation::ListObjectsInput;
use axum_s3::operation::ListObjectsOutput;
use axum_s3::operation::ListObjectsV2Input;
use axum_s3::operation::ListObjectsV2Output;
use axum_s3::operation::PutObjectInput;
use axum_s3::operation::PutObjectOutput;
use axum_s3::utils::ErrorParts;
use base64::Engine;
use digest::Digest;
use ensure::fixme;
use futures::TryStreamExt;
use minil_migration::Migrator;
use minil_migration::MigratorTrait;
use minil_service::BucketMutation;
use minil_service::BucketQuery;
use minil_service::ObjectMutation;
use minil_service::ObjectQuery;
use minil_service::OwnerQuery;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DbConn;
use sea_orm::TransactionTrait;
use serde_s3::operation::CreateBucketOutputHeader;
use serde_s3::operation::GetBucketLocationOutputBody;
use serde_s3::operation::GetBucketVersioningOutputBody;
use serde_s3::operation::HeadBucketOutputHeader;
use serde_s3::operation::ListBucketsOutputBody;
use serde_s3::operation::ListObjectsOutputBody;
use serde_s3::operation::ListObjectsOutputHeader;
use serde_s3::operation::ListObjectsV2OutputBody;
use serde_s3::operation::ListObjectsV2OutputHeader;
use serde_s3::operation::PutObjectOutputHeader;
use serde_s3::types::Bucket;
use serde_s3::types::BucketLocationConstraint;
use serde_s3::types::Object;
use serde_s3::types::Owner;
use sha2::Sha256;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;
use tower::Layer;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::normalize_path::NormalizePathLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::database_transaction::DbTxn;
use crate::error::AppError;
use crate::error::AppErrorDiscriminants;
use crate::error::AppResult;
use crate::macros::app_define_handler;
use crate::macros::app_define_routes;
use crate::macros::app_ensure_eq;
use crate::macros::app_ensure_matches;
use crate::macros::app_output;
use crate::macros::app_validate_digest;
use crate::macros::app_validate_owner;
use crate::make_request_id::AppMakeRequestId;
use crate::service_builder_ext::AppServiceBuilderExt;
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
        .expect("failed to set global default subscriber");

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_err| "sqlite::memory:".to_owned());
    tracing::info!("connecting to {}", db_url);
    let mut db_opt = ConnectOptions::new(db_url);
    db_opt.sqlx_logging(false);
    let db_conn = Database::connect(db_opt)
        .await
        .expect("failed to connect to database");
    Migrator::up(&db_conn, None)
        .await
        .expect("failed to run migrations");

    let state = AppState { db_conn };
    let node_id = format!("{:x}", Sha256::digest(NODE_NAME.as_bytes()));
    let server = format!("{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let middleware = ServiceBuilder::new()
        .trace_for_http()
        .decompression()
        .compression()
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
        "/{Bucket}" => put(create_bucket),
        "/{Bucket}/versioning" => get(get_bucket_versioning),
        "/{Bucket}/{*Key}" => put(put_object),
    });
    let router = router
        .method_not_allowed_fallback(async || AppError::MethodNotAllowed)
        .with_state(state)
        .layer(middleware);
    let router = ServiceExt::<Request>::into_make_service(
        NormalizePathLayer::trim_trailing_slash().layer(router),
    );

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000);
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    tracing::info!("listening on {}", addr);
    axum::serve(listener, router)
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
    if let Some(err) = err {
        err.into_response(err_parts)
    } else {
        response
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
        tracing::debug!("committing transaction");
        db_txn.commit().await?;
    } else {
        tracing::debug!("rolling back transaction");
        db_txn.rollback().await?;
    }

    Ok(response)
}

async fn create_bucket(
    Extension(db): Extension<DbTxn>,
    input: CreateBucketInput,
) -> AppResult<CreateBucketOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);
    app_ensure_matches!(input.header.acl, None);
    app_ensure_matches!(input.header.bucket_object_lock_enabled, None | Some(false));
    app_ensure_eq!(input.header.grant_full_control, None);
    app_ensure_eq!(input.header.grant_read, None);
    app_ensure_eq!(input.header.grant_read_acp, None);
    app_ensure_eq!(input.header.grant_write, None);
    app_ensure_eq!(input.header.grant_write_acp, None);
    app_ensure_matches!(input.header.object_ownership, None);
    app_ensure_matches!(input.body, None);

    let region = serde_plain::to_string(&BucketLocationConstraint::UsEast1)
        .unwrap_or_else(|_| unreachable!());
    let bucket = BucketMutation::create(db.as_ref(), owner.id, &input.path.bucket, &region)
        .await?
        .ok_or(AppError::BucketAlreadyOwnedByYou)?;

    app_output!(
        CreateBucketOutput::builder()
            .header(
                CreateBucketOutputHeader::builder()
                    .location(format!("/{}", bucket.name))
                    .build(),
            )
            .build()
    )
}

async fn delete_bucket(
    Extension(db): Extension<DbTxn>,
    input: DeleteBucketInput,
) -> AppResult<DeleteBucketOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    BucketMutation::delete_by_unique_id(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    app_output!(DeleteBucketOutput::builder().build())
}

async fn head_bucket(
    Extension(db): Extension<DbTxn>,
    input: HeadBucketInput,
) -> AppResult<HeadBucketOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find_by_unique_id(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;

    app_output!(
        HeadBucketOutput::builder()
            .header(
                HeadBucketOutputHeader::builder()
                    .bucket_region(bucket.region)
                    .build(),
            )
            .build()
    )
}

async fn list_buckets(
    Extension(db): Extension<DbTxn>,
    input: ListBucketsInput,
) -> AppResult<ListBucketsOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);

    let limit = input.query.max_buckets + 1;
    let mut buckets = BucketQuery::find_all_by_owner_id(
        db.as_ref(),
        owner.id,
        input.query.prefix.as_deref(),
        input.query.continuation_token.as_deref(),
        Some(limit as u64),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let continuation_token = if buckets.len() == limit as usize {
        Some(buckets.pop().unwrap_or_else(|| unreachable!()).name)
    } else {
        None
    };

    app_output!(
        ListBucketsOutput::builder()
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
            .build()
    )
}

async fn put_object(
    Extension(db): Extension<DbTxn>,
    input: PutObjectInput,
) -> AppResult<PutObjectOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);
    app_ensure_matches!(input.header.cache_control, None);
    app_ensure_matches!(input.header.content_disposition, None);
    app_ensure_matches!(input.header.content_encoding, None);
    app_ensure_matches!(input.header.content_language, None);
    app_ensure_matches!(input.header.expires, None);
    app_ensure_matches!(input.header.if_match, None);
    app_ensure_matches!(input.header.if_none_match, None);
    app_ensure_matches!(input.header.acl, None);
    app_ensure_matches!(input.header.grant_full_control, None);
    app_ensure_matches!(input.header.grant_read, None);
    app_ensure_matches!(input.header.grant_read_acp, None);
    app_ensure_matches!(input.header.grant_write_acp, None);
    app_ensure_matches!(input.header.object_lock_legal_hold, None);
    app_ensure_matches!(input.header.object_lock_mode, None);
    app_ensure_matches!(input.header.object_lock_retain_until_date, None);
    app_ensure_matches!(input.header.request_payer, None);
    app_ensure_matches!(input.header.sdk_checksum_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption, None);
    app_ensure_matches!(input.header.server_side_encryption_aws_kms_key_id, None);
    app_ensure_matches!(input.header.server_side_encryption_bucket_key_enabled, None);
    app_ensure_matches!(input.header.server_side_encryption_context, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_algorithm, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key, None);
    app_ensure_matches!(input.header.server_side_encryption_customer_key_md5, None);
    app_ensure_matches!(input.header.storage_class, None);
    app_ensure_matches!(input.header.tagging, None);
    app_ensure_matches!(input.header.website_redirect_location, None);
    app_ensure_matches!(input.header.write_offset_bytes, None | Some(0));

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find_by_unique_id(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let object = ObjectMutation::create(
        db.as_ref(),
        bucket.id,
        &input.path.key,
        input.header.content_type.as_deref(),
        StreamReader::new(
            input
                .body
                .into_data_stream()
                .map(|res| res.map_err(|err| io::Error::other(err.into_inner()))),
        ),
    )
    .await??;
    app_validate_digest!(input.header.content_md5, object.md5.clone());
    app_validate_digest!(input.header.checksum_crc32, object.crc32);
    app_validate_digest!(input.header.checksum_crc32c, object.crc32c);
    app_validate_digest!(input.header.checksum_crc64nvme, object.crc64nvme);
    app_validate_digest!(input.header.checksum_sha1, object.sha1);
    app_validate_digest!(input.header.checksum_sha256, object.sha256);

    app_output!(
        PutObjectOutput::builder()
            .header(
                PutObjectOutputHeader::builder()
                    .e_tag(format!("\"{}\"", hex::encode(object.md5)))
                    .build(),
            )
            .build()
    )
}

app_define_handler!(get_bucket_handler {
    GetBucketVersioningCheck => get_bucket_versioning,
    GetBucketLocationCheck => get_bucket_location,
    ListObjectsV2Check => list_objects_v2,
    _ => list_objects,
});

async fn get_bucket_versioning(
    Extension(db): Extension<DbTxn>,
    input: GetBucketVersioningInput,
) -> AppResult<GetBucketVersioningOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);

    app_output!(
        GetBucketVersioningOutput::builder()
            .body(GetBucketVersioningOutputBody::builder().build())
            .build()
    )
}

async fn get_bucket_location(
    Extension(db): Extension<DbTxn>,
    input: GetBucketLocationInput,
) -> AppResult<GetBucketLocationOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find_by_unique_id(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let content = serde_plain::from_str::<BucketLocationConstraint>(&bucket.region)
        .unwrap_or_else(|_err| unreachable!());

    app_output!(
        GetBucketLocationOutput::builder()
            .body(
                GetBucketLocationOutputBody::builder()
                    .content(content)
                    .build(),
            )
            .build()
    )
}

async fn list_objects(
    Extension(db): Extension<DbTxn>,
    input: ListObjectsInput,
) -> AppResult<ListObjectsOutput> {
    let owner = OwnerQuery::find_by_unique_id(db.as_ref(), "minil")
        .await?
        .unwrap();

    dbg!(&input);
    app_ensure_eq!(input.query.delimiter, None);
    app_ensure_matches!(input.query.encoding_type, None);
    app_ensure_matches!(input.header.optional_object_attributes, None);
    app_ensure_matches!(input.header.request_payer, None);

    app_validate_owner!(input.header.expected_bucket_owner, owner.name);
    let bucket = BucketQuery::find_by_unique_id(db.as_ref(), owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let limit = input.query.max_keys + 1;
    let mut objects = ObjectQuery::find_all_by_bucket_id(
        db.as_ref(),
        bucket.id,
        input.query.prefix.as_deref(),
        input.query.marker.as_deref(),
        Some(limit as u64),
    )
    .await?
    .try_collect::<Vec<_>>()
    .await?;
    let next_marker = if objects.len() == limit as usize {
        Some(objects.pop().unwrap_or_else(|| unreachable!()).key)
    } else {
        None
    };

    app_output!(
        ListObjectsOutput::builder()
            .header(ListObjectsOutputHeader::builder().build())
            .body(
                ListObjectsOutputBody::builder()
                    .common_prefixes(vec![])
                    .contents(
                        objects
                            .into_iter()
                            .map(|object| {
                                Object::builder()
                                    .e_tag(format!("\"{}\"", hex::encode(object.md5)))
                                    .key(object.key)
                                    .maybe_last_modified(object.updated_at)
                                    .owner(
                                        Owner::builder()
                                            .display_name(owner.name.clone())
                                            .id(owner.id)
                                            .build(),
                                    )
                                    .size(object.size as u64)
                                    .build()
                            })
                            .collect()
                    )
                    .is_truncated(next_marker.is_some())
                    .marker(input.query.marker.unwrap_or_default())
                    .max_keys(input.query.max_keys)
                    .name(bucket.name)
                    .maybe_next_marker(next_marker)
                    .prefix(input.query.prefix.unwrap_or_default())
                    .build(),
            )
            .build()
    )
}

async fn list_objects_v2(
    Extension(_db): Extension<DbTxn>,
    input: ListObjectsV2Input,
) -> AppResult<ListObjectsV2Output> {
    dbg!(&input);
    fixme!();

    app_output!(
        ListObjectsV2Output::builder()
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
            .build()
    )
}
