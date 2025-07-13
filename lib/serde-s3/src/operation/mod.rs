mod create_bucket;
mod delete_bucket;
mod get_bucket_location;
mod get_bucket_versioning;
mod get_object;
mod head_bucket;
mod head_object;
mod list_buckets;
mod list_objects;
mod list_objects_v2;
mod put_object;

pub use create_bucket::*;
pub use delete_bucket::*;
pub use get_bucket_location::*;
pub use get_bucket_versioning::*;
pub use get_object::*;
pub use head_bucket::*;
pub use head_object::*;
pub use list_buckets::*;
pub use list_objects::*;
pub use list_objects_v2::*;
pub use put_object::*;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use aws_sdk_s3::operation::copy_object::CopyObjectInput;
    use aws_sdk_s3::operation::copy_object::CopyObjectOutput;
    use aws_sdk_s3::operation::create_bucket::CreateBucketInput;
    use aws_sdk_s3::operation::create_bucket::CreateBucketOutput;
    use aws_sdk_s3::operation::delete_bucket::DeleteBucketInput;
    use aws_sdk_s3::operation::delete_bucket::DeleteBucketOutput;
    use aws_sdk_s3::operation::delete_object::DeleteObjectInput;
    use aws_sdk_s3::operation::delete_object::DeleteObjectOutput;
    use aws_sdk_s3::operation::delete_objects::DeleteObjectsInput;
    use aws_sdk_s3::operation::delete_objects::DeleteObjectsOutput;
    use aws_sdk_s3::operation::get_bucket_location::GetBucketLocationInput;
    use aws_sdk_s3::operation::get_bucket_location::GetBucketLocationOutput;
    use aws_sdk_s3::operation::get_bucket_versioning::GetBucketVersioningInput;
    use aws_sdk_s3::operation::get_bucket_versioning::GetBucketVersioningOutput;
    use aws_sdk_s3::operation::get_object::GetObjectInput;
    use aws_sdk_s3::operation::get_object::GetObjectOutput;
    use aws_sdk_s3::operation::head_bucket::HeadBucketInput;
    use aws_sdk_s3::operation::head_bucket::HeadBucketOutput;
    use aws_sdk_s3::operation::head_object::HeadObjectInput;
    use aws_sdk_s3::operation::head_object::HeadObjectOutput;
    use aws_sdk_s3::operation::list_buckets::ListBucketsInput;
    use aws_sdk_s3::operation::list_buckets::ListBucketsOutput;
    use aws_sdk_s3::operation::list_objects::ListObjectsInput;
    use aws_sdk_s3::operation::list_objects::ListObjectsOutput;
    use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Input;
    use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
    use aws_sdk_s3::operation::put_bucket_versioning::PutBucketVersioningInput;
    use aws_sdk_s3::operation::put_bucket_versioning::PutBucketVersioningOutput;
    use aws_sdk_s3::operation::put_object::PutObjectInput;
    use aws_sdk_s3::operation::put_object::PutObjectOutput;
}
