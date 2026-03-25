use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use serde::Deserialize;

use crate::domain::FolderName;
use crate::forms::FormError;

/// Form representing a single file upload.
#[derive(MultipartForm)]
pub struct UploadFileForm {
    /// Uploaded file with a 10MB limit.
    #[multipart(limit = "10MB")]
    pub file: TempFile,
}

/// Form data for creating a new folder.
#[derive(Deserialize)]
pub struct CreateFolderForm {
    /// Name of the folder to create. Must be at least one character long.
    pub name: String,
}

#[derive(Debug)]
pub struct CreateFolderPayload {
    pub name: FolderName,
}

impl TryFrom<CreateFolderForm> for CreateFolderPayload {
    type Error = FormError;

    fn try_from(form: CreateFolderForm) -> Result<Self, Self::Error> {
        let trimmed = form.name.trim();
        if trimmed.is_empty() {
            return Err(FormError::MissingFolderName);
        }

        Ok(Self {
            name: FolderName::try_from_str(trimmed).map_err(|_| FormError::InvalidFolderName)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{CreateFolderForm, CreateFolderPayload};

    #[test]
    fn create_folder_form_rejects_blank_name() {
        let form = CreateFolderForm {
            name: "   ".to_string(),
        };

        let errors = CreateFolderPayload::try_from(form).unwrap_err();
        assert_eq!(errors.to_string(), "Введите название папки.");
    }

    #[test]
    fn create_folder_form_rejects_parent_paths() {
        let form = CreateFolderForm {
            name: "../outside".to_string(),
        };

        let errors = CreateFolderPayload::try_from(form).unwrap_err();
        assert_eq!(errors.to_string(), "Недопустимое имя папки.");
    }

    #[test]
    fn create_folder_form_trims_valid_name() {
        let form = CreateFolderForm {
            name: "  media  ".to_string(),
        };

        assert_eq!(
            CreateFolderPayload::try_from(form).unwrap().name.as_str(),
            "media"
        );
    }

    #[test]
    fn create_folder_form_rejects_nested_names() {
        let form = CreateFolderForm {
            name: "media/photos".to_string(),
        };

        let errors = CreateFolderPayload::try_from(form).unwrap_err();
        assert_eq!(errors.to_string(), "Недопустимое имя папки.");
    }
}
