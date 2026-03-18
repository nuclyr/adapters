use async_trait::async_trait;
use bytes::Bytes;

use crate::error::AdapterError;
use crate::types::{ObjectMeta, PresignOperation, StorageOptions};

/// Trait that every storage provider adapter must implement
#[async_trait]
pub trait StorageAdapter: Send + Sync {
    /// Upload an object to storage
    async fn upload(
        &self,
        bucket: &str,
        key: &str,
        content: Bytes,
        content_type: Option<&str>,
        options: &StorageOptions,
    ) -> Result<ObjectMeta, AdapterError>;

    /// Download an object from storage
    async fn download(&self, bucket: &str, key: &str) -> Result<(Bytes, ObjectMeta), AdapterError>;

    /// Delete an object
    async fn delete(&self, bucket: &str, key: &str) -> Result<bool, AdapterError>;

    /// List objects with a prefix
    async fn list(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        max_results: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<ObjectMeta>, Option<String>), AdapterError>;

    /// Get object metadata without downloading
    async fn get_metadata(&self, bucket: &str, key: &str) -> Result<ObjectMeta, AdapterError>;

    /// Generate a presigned URL
    async fn presign(
        &self,
        bucket: &str,
        key: &str,
        operation: PresignOperation,
        expires_in_seconds: u32,
    ) -> Result<String, AdapterError>;
}
