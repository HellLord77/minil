mod error;
mod macros;
mod make_request_id;
mod service_builder_ext;
mod state;

use std::env;
use std::future;
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::time::Instant;

use axum::Router;
use axum::ServiceExt;
use axum::extract::Request;
use axum::extract::State;
use axum::handler::Handler;
use axum::http::HeaderName;
use axum::http::HeaderValue;
use axum::http::header;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::head;
use axum::routing::put;
use axum_extra::vpath;
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
use axum_s3::utils::GetBucketLocationCheck;
use axum_s3::utils::GetBucketVersioningCheck;
use axum_s3::utils::ListObjectsV2Check;
use crc_fast::CrcAlgorithm;
use ensure::ensure_eq;
use ensure::ensure_matches;
use ensure::fixme;
use md5::Md5;
use minil_migration::Migrator;
use minil_migration::MigratorTrait;
use minil_service::BucketMutation;
use minil_service::BucketQuery;
use minil_service::OwnerQuery;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DbConn;
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
use serde_s3::types::Owner;
use sha1::Sha1;
use sha2::Digest;
use sha2::Sha256;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_stream::StreamExt;
use tower::Layer;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::normalize_path::NormalizePathLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::error::AppError;
use crate::error::AppErrorDiscriminants;
use crate::error::AppResult;
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
        .expect("unable to install global subscriber");

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
        .middleware_fn(set_process_time)
        .decompression()
        .set_request_id(REQUEST_ID_HEADER, AppMakeRequestId)
        .middleware_fn(handle_app_error)
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
        .compression();

    let router = Router::new()
        .route(vpath!("/"), get(list_buckets))
        .route(vpath!("/{Bucket}"), delete(delete_bucket))
        .route(vpath!("/{Bucket}"), get(get_bucket_handler))
        .route(vpath!("/{Bucket}"), head(head_bucket))
        .route(vpath!("/{Bucket}"), put(create_bucket))
        .route(vpath!("/{Bucket}/versioning"), get(get_bucket_versioning))
        .route(vpath!("/{Bucket}/{*Key}"), put(put_object))
        .with_state(state)
        .layer(middleware);

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
    let (parts, body) = request.into_parts();
    let request = Request::from_parts(parts.clone(), body);
    let mut response = next.run(request).await;

    if let Some(err) = response.extensions().get::<AppErrorDiscriminants>() {
        response = err.into_response(&parts);
    }

    response
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

    let region = serde_plain::to_string(&BucketLocationConstraint::UsEast1)
        .unwrap_or_else(|_| unreachable!());
    let bucket = BucketMutation::create(&db, owner.id, &input.path.bucket, region.as_str())
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
            Err(AppError::Forbidden)?
        }
    }
    BucketMutation::delete_by_unique_id(&db, owner.id, &input.path.bucket)
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
            Err(AppError::Forbidden)?
        }
    }
    let bucket = BucketQuery::find_by_unique_id(&db, owner.id, &input.path.bucket)
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

async fn put_object(State(db): State<DbConn>, input: PutObjectInput) -> AppResult<PutObjectOutput> {
    let owner = OwnerQuery::find_by_unique_id(&db, "minil").await?.unwrap();

    dbg!(&input);
    fixme!();
    ensure_matches!(input.header.cache_control, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.content_disposition,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.content_encoding,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.content_language,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(input.header.content_md5, None, AppError::NotImplemented);
    ensure_matches!(input.header.content_type, None, AppError::NotImplemented);
    ensure_matches!(input.header.expires, None, AppError::NotImplemented);
    ensure_matches!(input.header.if_match, None, AppError::NotImplemented);
    ensure_matches!(input.header.if_none_match, None, AppError::NotImplemented);
    ensure_matches!(input.header.acl, None, AppError::NotImplemented);
    ensure_matches!(input.header.checksum_crc32, None, AppError::NotImplemented);
    ensure_matches!(input.header.checksum_crc32c, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.checksum_crc64nvme,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(input.header.checksum_sha1, None, AppError::NotImplemented);
    ensure_matches!(input.header.checksum_sha256, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.grant_full_control,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(input.header.grant_read, None, AppError::NotImplemented);
    ensure_matches!(input.header.grant_read_acp, None, AppError::NotImplemented);
    ensure_matches!(input.header.grant_write_acp, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.object_lock_legal_hold,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.object_lock_mode,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.object_lock_retain_until_date,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(input.header.request_payer, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.sdk_checksum_algorithm,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.server_side_encryption,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.server_side_encryption_aws_kms_key_id,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.server_side_encryption_bucket_key_enabled,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.server_side_encryption_context,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.server_side_encryption_customer_algorithm,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.server_side_encryption_customer_key,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.server_side_encryption_customer_key_md5,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(input.header.storage_class, None, AppError::NotImplemented);
    ensure_matches!(input.header.tagging, None, AppError::NotImplemented);
    ensure_matches!(
        input.header.website_redirect_location,
        None,
        AppError::NotImplemented
    );
    ensure_matches!(
        input.header.write_offset_bytes,
        None | Some(0),
        AppError::NotImplemented
    );

    if let Some(expected_bucket_owner) = input.header.expected_bucket_owner {
        if expected_bucket_owner != owner.name {
            Err(AppError::Forbidden)?
        }
    }
    let mut stream = input.body.into_data_stream();
    use crc_fast::Digest;
    let mut crc32 = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
    let mut crc32c = Digest::new(CrcAlgorithm::Crc32Iscsi);
    let mut crc64nvme = Digest::new(CrcAlgorithm::Crc64Nvme);
    let mut sha1 = Sha1::new();
    let mut sha256 = Sha256::new();
    let mut md5 = Md5::new();
    while let Some(chunk) = stream.try_next().await? {
        dbg!(&chunk);
        crc32.update(&chunk);
        crc32c.update(&chunk);
        crc64nvme.update(&chunk);
        sha1.update(&chunk);
        sha256.update(&chunk);
        md5.update(&chunk);
    }
    dbg!(crc32.finalize());
    dbg!(crc32c.finalize());
    dbg!(crc64nvme.finalize());
    dbg!(sha1.finalize());
    dbg!(sha256.finalize());
    dbg!(md5.finalize());

    let output = PutObjectOutput::builder()
        .header(
            PutObjectOutputHeader::builder()
                .e_tag("".to_owned())
                .build(),
        )
        .build();

    dbg!(&output);
    Ok(output)
}

async fn get_bucket_handler(
    maybe_get_bucket_versioning: Option<GetBucketVersioningCheck>,
    maybe_get_bucket_location: Option<GetBucketLocationCheck>,
    maybe_list_objects_v2: Option<ListObjectsV2Check>,
    State(state): State<AppState>,
    request: Request,
) -> Response {
    if maybe_get_bucket_versioning.is_some() {
        get_bucket_versioning.call(request, state).await
    } else if maybe_get_bucket_location.is_some() {
        get_bucket_location.call(request, state).await
    } else if maybe_list_objects_v2.is_some() {
        list_objects_v2.call(request, state).await
    } else {
        list_objects.call(request, state).await
    }
}

async fn get_bucket_versioning(
    State(db): State<DbConn>,
    input: GetBucketVersioningInput,
) -> AppResult<GetBucketVersioningOutput> {
    let owner = OwnerQuery::find_by_unique_id(&db, "minil").await?.unwrap();

    dbg!(&input);

    if let Some(expected_bucket_owner) = input.header.expected_bucket_owner {
        if expected_bucket_owner != owner.name {
            Err(AppError::Forbidden)?
        }
    }

    let output = GetBucketVersioningOutput::builder()
        .body(GetBucketVersioningOutputBody::builder().build())
        .build();

    dbg!(&output);
    Ok(output)
}

async fn get_bucket_location(
    State(db): State<DbConn>,
    input: GetBucketLocationInput,
) -> AppResult<GetBucketLocationOutput> {
    let owner = OwnerQuery::find_by_unique_id(&db, "minil").await?.unwrap();

    dbg!(&input);

    if let Some(expected_bucket_owner) = input.header.expected_bucket_owner {
        if expected_bucket_owner != owner.name {
            Err(AppError::Forbidden)?
        }
    }
    let bucket = BucketQuery::find_by_unique_id(&db, owner.id, &input.path.bucket)
        .await?
        .ok_or(AppError::NoSuchBucket)?;
    let content = serde_plain::from_str::<BucketLocationConstraint>(&bucket.region)
        .unwrap_or_else(|_err| unreachable!());

    let output = GetBucketLocationOutput::builder()
        .body(
            GetBucketLocationOutputBody::builder()
                .content(content)
                .build(),
        )
        .build();

    dbg!(&output);
    Ok(output)
}

async fn list_objects(State(_db): State<DbConn>, input: ListObjectsInput) -> impl IntoResponse {
    dbg!(&input);
    fixme!();

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

async fn list_objects_v2(
    State(_db): State<DbConn>,
    input: ListObjectsV2Input,
) -> impl IntoResponse {
    dbg!(&input);
    fixme!();

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

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use blake3::traits;
}
