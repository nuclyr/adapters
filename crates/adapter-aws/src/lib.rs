//! AWS adapter for Nuclyr - S3, Lambda, SQS implementations.

pub mod auth;
pub mod s3;

pub use s3::S3Adapter;
