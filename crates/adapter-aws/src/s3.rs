//! AWS S3 storage adapter.

use async_trait::async_trait;
use aws_sdk_s3::Client;
use bytes::Bytes;

use nuclyr_adapter_core::{
    AdapterError, ObjectMeta, PresignOperation, Provider, StorageAdapter, StorageOptions,
};

/// S3 implementation of the StorageAdapter trait.
pub struct S3Adapter {
    client: Client,
    region: String,
}

impl S3Adapter {
    pub fn new(client: Client, region: String) -> Self {
        Self { client, region }
    }
}

#[async_trait]
impl StorageAdapter for S3Adapter {
    async fn upload(
        &self,
        bucket: &str,
        key: &str,
        content: Bytes,
        content_type: Option<&str>,
        _options: &StorageOptions,
    ) -> Result<ObjectMeta, AdapterError> {
        let mut req = self
            .client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(content.into());

        if let Some(ct) = content_type {
            req = req.content_type(ct);
        }

        let resp = req.send().await.map_err(|e| AdapterError::ProviderError {
            provider: Provider::Aws,
            message: format!("S3 PutObject failed: {e}"),
            source: Some(Box::new(e)),
        })?;

        Ok(ObjectMeta {
            bucket: bucket.to_string(),
            key: key.to_string(),
            size_bytes: 0, // not returned by PutObject
            content_type: content_type.map(String::from),
            etag: resp.e_tag().map(String::from),
            provider: Provider::Aws,
            region: self.region.clone(),
        })
    }

    async fn download(&self, bucket: &str, key: &str) -> Result<(Bytes, ObjectMeta), AdapterError> {
        let resp = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AdapterError::ProviderError {
                provider: Provider::Aws,
                message: format!("S3 GetObject failed: {e}"),
                source: Some(Box::new(e)),
            })?;

        let meta = ObjectMeta {
            bucket: bucket.to_string(),
            key: key.to_string(),
            size_bytes: resp.content_length().unwrap_or(0) as u64,
            content_type: resp.content_type().map(String::from),
            etag: resp.e_tag().map(String::from),
            provider: Provider::Aws,
            region: self.region.clone(),
        };

        let body = resp
            .body
            .collect()
            .await
            .map_err(|e| AdapterError::ProviderError {
                provider: Provider::Aws,
                message: format!("S3 body read failed: {e}"),
                source: Some(Box::new(e)),
            })?
            .into_bytes();

        Ok((body, meta))
    }

    async fn delete(&self, bucket: &str, key: &str) -> Result<bool, AdapterError> {
        self.client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AdapterError::ProviderError {
                provider: Provider::Aws,
                message: format!("S3 DeleteObject failed: {e}"),
                source: Some(Box::new(e)),
            })?;

        Ok(true)
    }

    async fn list(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        max_results: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<ObjectMeta>, Option<String>), AdapterError> {
        let mut req = self.client.list_objects_v2().bucket(bucket);

        if let Some(p) = prefix {
            req = req.prefix(p);
        }
        if let Some(max) = max_results {
            req = req.max_keys(max as i32);
        }
        if let Some(token) = page_token {
            req = req.continuation_token(token);
        }

        let resp = req.send().await.map_err(|e| AdapterError::ProviderError {
            provider: Provider::Aws,
            message: format!("S3 ListObjectsV2 failed: {e}"),
            source: Some(Box::new(e)),
        })?;

        let objects = resp
            .contents()
            .iter()
            .map(|obj| ObjectMeta {
                bucket: bucket.to_string(),
                key: obj.key().unwrap_or_default().to_string(),
                size_bytes: obj.size().unwrap_or(0) as u64,
                content_type: None,
                etag: obj.e_tag().map(String::from),
                provider: Provider::Aws,
                region: self.region.clone(),
            })
            .collect();

        let next_token = resp.next_continuation_token().map(String::from);

        Ok((objects, next_token))
    }

    async fn get_metadata(&self, bucket: &str, key: &str) -> Result<ObjectMeta, AdapterError> {
        let resp = self
            .client
            .head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AdapterError::ProviderError {
                provider: Provider::Aws,
                message: format!("S3 HeadObject failed: {e}"),
                source: Some(Box::new(e)),
            })?;

        Ok(ObjectMeta {
            bucket: bucket.to_string(),
            key: key.to_string(),
            size_bytes: resp.content_length().unwrap_or(0) as u64,
            content_type: resp.content_type().map(String::from),
            etag: resp.e_tag().map(String::from),
            provider: Provider::Aws,
            region: self.region.clone(),
        })
    }

    async fn presign(
        &self,
        _bucket: &str,
        _key: &str,
        _operation: PresignOperation,
        _expires_in_seconds: u32,
    ) -> Result<String, AdapterError> {
        // TODO: implement presigning with aws_sdk_s3::presigning
        Err(AdapterError::Unsupported {
            provider: Provider::Aws,
            operation: "presign (not yet implemented)".into(),
        })
    }
}
