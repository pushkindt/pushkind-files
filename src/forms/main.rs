use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use serde::Deserialize;
use validator::Validate;

/// Form representing a single file upload.
#[derive(MultipartForm)]
pub struct UploadFileForm {
    /// Uploaded file with a 10MB limit.
    #[multipart(limit = "10MB")]
    pub file: TempFile,
}

/// Form data for creating a new folder.
#[derive(Deserialize, Validate)]
pub struct CreateFolderForm {
    /// Name of the folder to create. Must be at least one character long.
    #[validate(length(min = 1))]
    pub name: String,
}
