//! Provide Error types
pub use caretta_sync_core::error::CoreError;
#[cfg(feature="service")]
pub use caretta_sync_service::error::ServiceError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[cfg(feature="service")]
    #[error(transparent)]
    Service(#[from] ServiceError),
}
