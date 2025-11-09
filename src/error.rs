pub use caretta_sync_core::error::CoreError;
pub use caretta_sync_service::error::ServiceError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error(transparent)]
    Service(#[from] ServiceError),
}