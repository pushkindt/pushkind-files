use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};

#[derive(MultipartForm)]
pub struct UploadFileForm {
    #[multipart(limit = "10MB")]
    pub image: TempFile,
    pub path: Text<String>,
}
