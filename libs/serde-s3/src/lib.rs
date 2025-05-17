pub mod create_bucket;
pub mod delete_bucket;
pub mod list_buckets;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use aws_sdk_s3::operation::{
        create_bucket::{CreateBucketInput, CreateBucketOutput},
        delete_bucket::{DeleteBucketInput, DeleteBucketOutput},
        list_buckets::{ListBucketsInput, ListBucketsOutput},
    };
}
