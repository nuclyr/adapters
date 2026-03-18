use serde::{Deserialize, Serialize};

/// Cloud provider identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Provider {
    Aws,
    Gcp,
    Azure,
}

/// Data residency constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataResidency {
    /// Data must stay in India
    IndiaOnly,
    /// No restriction
    Any,
}

/// Object metadata returned from storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMeta {
    pub bucket: String,
    pub key: String,
    pub size_bytes: u64,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub provider: Provider,
    pub region: String,
}

/// Options for storage operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageOptions {
    pub residency: Option<DataResidency>,
    pub tags: Option<std::collections::HashMap<String, String>>,
}

/// Options for compute operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComputeOptions {
    pub timeout_seconds: Option<u32>,
    pub memory_mb: Option<u32>,
    pub residency: Option<DataResidency>,
    pub env: Option<std::collections::HashMap<String, String>>,
}

/// Compute job status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatus {
    pub job_id: String,
    pub state: JobState,
    pub provider: Provider,
    pub region: String,
}

/// Compute job state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Queue message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMessage {
    pub message_id: String,
    pub payload: Vec<u8>,
    pub attributes: std::collections::HashMap<String, String>,
}

/// Options for queue operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueueOptions {
    pub delay_seconds: Option<u32>,
    pub residency: Option<DataResidency>,
}

/// Presign operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresignOperation {
    Get,
    Put,
}

/// Credentials for authenticating with a cloud provider
#[derive(Debug, Clone)]
pub enum CloudCredentials {
    Aws(AwsCredentials),
    Gcp(GcpCredentials),
    Azure(AzureCredentials),
}

#[derive(Debug, Clone)]
pub struct AwsCredentials {
    pub role_arn: String,
    pub external_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GcpCredentials {
    pub project_id: String,
    pub service_account_json: String,
}

#[derive(Debug, Clone)]
pub struct AzureCredentials {
    pub tenant_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub subscription_id: String,
}
