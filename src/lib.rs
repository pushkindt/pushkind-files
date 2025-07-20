use std::path::{Path, PathBuf};

pub mod forms;
pub mod middleware;
pub mod models;
pub mod routes;

pub const UPLOAD_PATH: &str = "./upload/";

/// Returns `None` if the path is invalid (e.g., contains `..`)
/// Trims leading slashes
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

/// Returns `true` if the provided file name has a common image extension.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn sanitize_path_valid() {
        let path = sanitize_path("folder/sub").expect("valid path");
        assert_eq!(path, Path::new("folder/sub"));
    }

    #[test]
    fn sanitize_path_invalid_parent() {
        assert!(sanitize_path("../secret").is_none());
        assert!(sanitize_path("folder/../secret").is_none());
    }

    #[test]
    fn sanitize_path_leading_slash() {
        let path = sanitize_path("/leading/path").expect("valid path");
        assert_eq!(path, Path::new("leading/path"));
    }

    #[test]
    fn is_image_file_positive() {
        assert!(is_image_file("photo.JPG"));
        assert!(is_image_file("image.png"));
    }

    #[test]
    fn is_image_file_negative() {
        assert!(!is_image_file("document.txt"));
        assert!(!is_image_file("noextension"));
    }
}
