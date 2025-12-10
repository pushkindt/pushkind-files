//! Application services orchestrating domain logic and side effects.
pub mod files;

/// Convenience alias for service results.
pub type ServiceResult<T> = Result<T, ServiceError>;

/// Errors surfaced by service operations.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("missing required role")]
    Unauthorized,
    #[error("invalid form input: {0}")]
    Validation(String),
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
