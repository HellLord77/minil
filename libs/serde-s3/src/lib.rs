pub mod create_bucket;
pub mod delete_bucket;
pub mod list_buckets;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use aws_sdk_s3::{
        operation::create_bucket::{CreateBucketInput, CreateBucketOutput},
        operation::delete_bucket::{DeleteBucketInput, DeleteBucketOutput},
        operation::list_buckets::{ListBucketsInput, ListBucketsOutput},
    };
}
