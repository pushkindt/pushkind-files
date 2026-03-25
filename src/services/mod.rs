//! Application services orchestrating domain logic and side effects.
pub mod api;
pub mod files;

use crate::forms::FormError;

/// Convenience alias for service results.
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Errors surfaced by service operations.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("missing required role")]
    Unauthorized,
    #[error("invalid form input: {0}")]
    Form(#[source] FormError),
    #[error("invalid path")]
    InvalidPath,
    #[error("invalid file name")]
    InvalidFileName,
    #[error("failed to prepare storage")]
    StorageSetup(#[source] std::io::Error),
    #[error("failed to list entries")]
    ListEntries(#[source] std::io::Error),
    #[error("failed to create folder")]
    CreateFolder(#[source] std::io::Error),
    #[error("failed to save file")]
    SaveFile(#[source] std::io::Error),
}
