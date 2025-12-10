//! Strongly-typed domain structures for file handling.
use std::fmt;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Identifier of a hub owning a storage root.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct HubId(i32);

impl HubId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl From<i32> for HubId {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for HubId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Absolute path to the upload root (e.g. `./upload`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UploadRoot(PathBuf);

impl UploadRoot {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl From<PathBuf> for UploadRoot {
    fn from(value: PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<UploadRoot> for PathBuf {
    fn from(value: UploadRoot) -> Self {
        value.0
    }
}

/// Path relative to a hub's storage root. Assumed sanitized.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RelativePath(PathBuf);

impl RelativePath {
    pub fn try_new(path: PathBuf) -> Result<Self, TypeConstraintError> {
        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(TypeConstraintError::InvalidPath);
        }

        Ok(Self(path))
    }

    pub fn try_from_str(input: &str) -> Result<Self, TypeConstraintError> {
        let trimmed = input.trim_start_matches('/');
        let path = Path::new(trimmed).to_path_buf();
        Self::try_new(path)
    }

    pub fn root() -> Self {
        Self(PathBuf::new())
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn join(&self, child: &RelativePath) -> RelativePath {
        let mut combined = self.0.clone();
        combined.push(child.as_path());
        RelativePath(combined)
    }
}

/// Sanitized file name (single path component).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FileName(String);

impl FileName {
    pub fn try_new(value: String) -> Result<Self, TypeConstraintError> {
        let path = Path::new(&value);
        let mut components = path.components();
        match (components.next(), components.next()) {
            (Some(std::path::Component::Normal(component)), None) => {
                Ok(Self(component.to_string_lossy().to_string()))
            }
            _ => Err(TypeConstraintError::InvalidFileName),
        }
    }

    pub fn try_from_str(value: &str) -> Result<Self, TypeConstraintError> {
        Self::try_new(value.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.0)
    }

    pub fn is_image(&self) -> bool {
        Path::new(&self.0)
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
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// File system entry recorded for a hub's storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageEntry {
    name: FileName,
    kind: EntryKind,
}

impl StorageEntry {
    pub fn new(name: FileName, kind: EntryKind) -> Self {
        Self { name, kind }
    }

    pub fn name(&self) -> &FileName {
        &self.name
    }

    pub fn into_name(self) -> FileName {
        self.name
    }

    pub fn is_directory(&self) -> bool {
        matches!(self.kind, EntryKind::Directory)
    }

    pub fn is_image(&self) -> bool {
        matches!(self.kind, EntryKind::File { is_image: true })
    }
}

/// Entry type stored on disk.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntryKind {
    Directory,
    File { is_image: bool },
}

/// Hub-scoped access to storage paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HubStorage {
    root: UploadRoot,
    hub_id: HubId,
}

impl HubStorage {
    pub fn new(root: UploadRoot, hub_id: HubId) -> Self {
        Self { root, hub_id }
    }

    /// Absolute path to the hub root (root + hub id).
    pub fn hub_root(&self) -> PathBuf {
        self.root.as_path().join(self.hub_id.to_string())
    }

    /// Resolve a relative path within the hub root.
    pub fn resolve_dir(&self, relative: &RelativePath) -> PathBuf {
        self.hub_root().join(relative.as_path())
    }

    /// Resolve a file name inside the given relative path.
    pub fn resolve_file(&self, relative: &RelativePath, name: &FileName) -> PathBuf {
        let mut path = self.resolve_dir(relative);
        path.push(name.as_str());
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hub_storage_resolves_paths() {
        let storage = HubStorage::new(UploadRoot::from(PathBuf::from("upload")), HubId::from(7));

        let relative = RelativePath::try_new(PathBuf::from("nested/path")).unwrap();
        let file_name = FileName::try_new("file.txt".to_string()).unwrap();
        let dir = storage.resolve_dir(&relative);
        let file = storage.resolve_file(&relative, &file_name);

        assert_eq!(dir, PathBuf::from("upload/7/nested/path"));
        assert_eq!(file, PathBuf::from("upload/7/nested/path/file.txt"));
    }

    #[test]
    fn relative_path_join_appends_segments() {
        let base = RelativePath::try_new(PathBuf::from("alpha")).unwrap();
        let child = RelativePath::try_new(PathBuf::from("beta")).unwrap();

        let combined = base.join(&child);
        assert_eq!(combined.as_path(), Path::new("alpha/beta"));
    }

    #[test]
    fn file_name_detects_images() {
        let png = FileName::try_new("photo.PNG".to_string()).unwrap();
        let txt = FileName::try_new("notes.txt".to_string()).unwrap();

        assert!(png.is_image());
        assert!(!txt.is_image());
    }

    #[test]
    fn file_name_rejects_nested() {
        assert!(FileName::try_new("foo/bar.txt".to_string()).is_err());
        assert!(FileName::try_new("../evil.txt".to_string()).is_err());
    }

    #[test]
    fn relative_path_rejects_parent() {
        assert!(RelativePath::try_new(PathBuf::from("../foo")).is_err());
    }
}

#[derive(Debug, Error)]
pub enum TypeConstraintError {
    #[error("invalid relative path")]
    InvalidPath,
    #[error("invalid file name")]
    InvalidFileName,
}
