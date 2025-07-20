use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use serde::Deserialize;

#[derive(MultipartForm)]
pub struct UploadFileForm {
    #[multipart(limit = "10MB")]
    pub image: TempFile,
    pub path: Text<String>,
}

#[derive(Deserialize)]
pub struct CreateFolderForm {
    pub name: String,
}
