use serde::Serialize;

/// Serializable entry for template rendering.
#[derive(Clone, Debug, Serialize)]
pub struct FileEntryDto {
    pub name: String,
    pub is_directory: bool,
    pub is_image: bool,
}

impl From<crate::domain::StorageEntry> for FileEntryDto {
    fn from(entry: crate::domain::StorageEntry) -> Self {
        let is_directory = entry.is_directory();
        let is_image = entry.is_image();
        let name = entry.into_name().into_string();

        Self {
            name,
            is_directory,
            is_image,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct FileBrowserEntryApiDto {
    pub name: String,
    pub is_directory: bool,
    pub is_image: bool,
    pub relative_path: String,
    pub navigation_path: Option<String>,
    pub download_url: Option<String>,
    pub copy_url: Option<String>,
    pub preview_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct FileBrowserDataDto {
    pub hub_id: i32,
    pub current_path: String,
    pub entries: Vec<FileBrowserEntryApiDto>,
}
