use async_trait::async_trait;
use bytes::Bytes;

use crate::error::AdapterError;
use crate::types::{QueueMessage, QueueOptions};

/// Trait that every queue provider adapter must implement
#[async_trait]
pub trait QueueAdapter: Send + Sync {
    /// Publish a message to a topic/queue
    async fn publish(
        &self,
        topic: &str,
        payload: Bytes,
        attributes: &std::collections::HashMap<String, String>,
        options: &QueueOptions,
    ) -> Result<String, AdapterError>; // returns message_id

    /// Receive messages from a subscription
    async fn receive(
        &self,
        topic: &str,
        subscription: &str,
        max_messages: u32,
    ) -> Result<Vec<QueueMessage>, AdapterError>;

    /// Acknowledge a message (mark as processed)
    async fn ack(&self, subscription: &str, message_id: &str) -> Result<bool, AdapterError>;
}
