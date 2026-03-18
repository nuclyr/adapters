//! GCP adapter for Nuclyr — Cloud Storage, Cloud Run, Pub/Sub implementations.
//!
//! TODO: Implement after AWS adapter is validated.
//! Will follow the same pattern as adapter-aws.

pub mod gcs;

pub use gcs::GcsAdapter;
