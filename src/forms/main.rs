use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use serde::Deserialize;

#[derive(MultipartForm)]
pub struct UploadFileForm {
    #[multipart(limit = "10MB")]
    pub file: TempFile,
}

#[derive(Deserialize)]
pub struct CreateFolderForm {
    pub name: String,
}
