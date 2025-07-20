use std::path::{Path, PathBuf};

pub mod forms;
pub mod middleware;
pub mod models;
pub mod routes;

pub const UPLOAD_PATH: &'static str = "./upload/";

/// Returns `None` if the path is invalid (e.g., contains `..`)
fn sanitize_path(input: &str) -> Option<PathBuf> {
    let trimmed = input.trim_start_matches('/');
    let path = Path::new(trimmed);

    // Reject paths with components that go up the directory tree
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return None;
    }

    Some(path.to_path_buf())
}
