use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use serde::Deserialize;
use validator::Validate;

#[derive(MultipartForm)]
pub struct UploadFileForm {
    #[multipart(limit = "10MB")]
    pub file: TempFile,
}

#[derive(Deserialize, Validate)]
pub struct CreateFolderForm {
    #[validate(length(min = 1))]
    pub name: String,
}
