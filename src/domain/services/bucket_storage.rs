use crate::domain::errors::bucket_error::BucketError;
use axum::async_trait;

#[async_trait]
pub trait BucketStorage: Send + Sync + 'static {
    async fn upload_file(&self, path: &str, bytes: Vec<u8>) -> Result<String, BucketError>;
    async fn delete_file(&self, path: &str) -> Result<(), BucketError>;
}
