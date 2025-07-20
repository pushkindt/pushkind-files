use std::path::{Path, PathBuf};

pub mod forms;
pub mod middleware;
pub mod models;
pub mod routes;

pub const UPLOAD_PATH: &str = "./upload/";

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

fn is_image_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg"
            )
        })
        .unwrap_or(false)
}
