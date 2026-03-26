//! GCP Cloud Storage adapter - stub implementation.

use async_trait::async_trait;
use bytes::Bytes;

use nuclyr_adapter_core::{
    AdapterError, ObjectMeta, PresignOperation, Provider, StorageAdapter, StorageOptions,
};

pub struct GcsAdapter;

#[async_trait]
impl StorageAdapter for GcsAdapter {
    async fn upload(
        &self,
        _bucket: &str,
        _key: &str,
        _content: Bytes,
        _content_type: Option<&str>,
        _options: &StorageOptions,
    ) -> Result<ObjectMeta, AdapterError> {
        Err(AdapterError::Unsupported {
            provider: Provider::Gcp,
            operation: "GCS upload (not yet implemented)".into(),
        })
    }

    async fn download(
        &self,
        _bucket: &str,
        _key: &str,
    ) -> Result<(Bytes, ObjectMeta), AdapterError> {
        Err(AdapterError::Unsupported {
            provider: Provider::Gcp,
            operation: "GCS download (not yet implemented)".into(),
        })
    }

    async fn delete(&self, _bucket: &str, _key: &str) -> Result<bool, AdapterError> {
        Err(AdapterError::Unsupported {
            provider: Provider::Gcp,
            operation: "GCS delete (not yet implemented)".into(),
        })
    }

    async fn list(
        &self,
        _bucket: &str,
        _prefix: Option<&str>,
        _max_results: Option<u32>,
        _page_token: Option<&str>,
    ) -> Result<(Vec<ObjectMeta>, Option<String>), AdapterError> {
        Err(AdapterError::Unsupported {
            provider: Provider::Gcp,
            operation: "GCS list (not yet implemented)".into(),
        })
    }

    async fn get_metadata(
        &self,
        _bucket: &str,
        _key: &str,
    ) -> Result<ObjectMeta, AdapterError> {
        Err(AdapterError::Unsupported {
            provider: Provider::Gcp,
            operation: "GCS get_metadata (not yet implemented)".into(),
        })
    }

    async fn presign(
        &self,
        _bucket: &str,
        _key: &str,
        _operation: PresignOperation,
        _expires_in_seconds: u32,
    ) -> Result<String, AdapterError> {
        Err(AdapterError::Unsupported {
            provider: Provider::Gcp,
            operation: "GCS presign (not yet implemented)".into(),
        })
    }
}
