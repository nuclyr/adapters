use async_trait::async_trait;
use bytes::Bytes;

use crate::error::AdapterError;
use crate::types::{ComputeOptions, JobStatus};

/// Trait that every compute provider adapter must implement
#[async_trait]
pub trait ComputeAdapter: Send + Sync {
    /// Run a compute function/job
    async fn run(
        &self,
        function_name: &str,
        payload: Bytes,
        options: &ComputeOptions,
    ) -> Result<(String, Bytes), AdapterError>; // (job_id, result)

    /// Get the status of a running job
    async fn get_status(&self, job_id: &str) -> Result<JobStatus, AdapterError>;

    /// Cancel a running job
    async fn cancel(&self, job_id: &str) -> Result<bool, AdapterError>;
}
