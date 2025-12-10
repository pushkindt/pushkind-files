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
