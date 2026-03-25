use pushkind_common::domain::auth::AuthenticatedUser;
use serde::{Deserialize, Serialize};

use crate::forms::FormError;

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
pub struct CurrentUserDto {
    pub email: String,
    pub name: String,
    pub hub_id: i32,
}

impl From<&AuthenticatedUser> for CurrentUserDto {
    fn from(user: &AuthenticatedUser) -> Self {
        Self {
            email: user.email.clone(),
            name: user.name.clone(),
            hub_id: user.hub_id,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct FilesShellDto {
    pub current_user: CurrentUserDto,
    pub home_url: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct NoAccessPageDto {
    pub current_user: CurrentUserDto,
    pub home_url: String,
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ApiFieldErrorDto {
    pub field: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ApiMutationSuccessDto {
    pub message: String,
    pub redirect_to: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ApiMutationErrorDto {
    pub message: String,
    pub field_errors: Vec<ApiFieldErrorDto>,
}

impl Default for ApiMutationErrorDto {
    fn default() -> Self {
        Self {
            message: "Ошибка валидации формы.".to_string(),
            field_errors: Vec::new(),
        }
    }
}

impl From<&FormError> for ApiMutationErrorDto {
    fn from(error: &FormError) -> Self {
        Self {
            message: "Ошибка валидации формы.".to_string(),
            field_errors: error
                .field_errors()
                .into_iter()
                .map(|error| ApiFieldErrorDto {
                    field: error.field.into_owned(),
                    message: error.message.into_owned(),
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiMutationErrorDto, CurrentUserDto};
    use crate::forms::FormError;
    use pushkind_common::domain::auth::AuthenticatedUser;

    #[test]
    fn mutation_error_from_form_error_preserves_field_errors() {
        let dto = ApiMutationErrorDto::from(&FormError::MissingFolderName);

        assert_eq!(dto.message, "Ошибка валидации формы.");
        assert_eq!(dto.field_errors.len(), 1);
        assert_eq!(dto.field_errors[0].field, "name");
        assert_eq!(dto.field_errors[0].message, "Введите название папки.");
    }

    #[test]
    fn current_user_dto_can_be_built_from_authenticated_user_reference() {
        let user = AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 42,
            name: "User".into(),
            roles: vec!["files".into()],
            exp: 0,
        };

        let dto = CurrentUserDto::from(&user);

        assert_eq!(dto.email, "user@example.com");
        assert_eq!(dto.name, "User");
        assert_eq!(dto.hub_id, 42);
    }
}
