//! Core trait definitions for Nuclyr cloud provider adapters.
//!
//! Each cloud provider (AWS, GCP, Azure) implements these traits
//! to provide a unified interface for the Nuclyr engine.

pub mod compute;
pub mod error;
pub mod queue;
pub mod storage;
pub mod types;

pub use compute::ComputeAdapter;
pub use error::AdapterError;
pub use queue::QueueAdapter;
pub use storage::StorageAdapter;
pub use types::*;
