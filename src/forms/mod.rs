use std::borrow::Cow;

use thiserror::Error;

pub mod main;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormFieldError {
    pub field: Cow<'static, str>,
    pub message: Cow<'static, str>,
}

#[derive(Debug, Error)]
pub enum FormError {
    #[error("Введите название папки.")]
    MissingFolderName,

    #[error("Недопустимое имя папки.")]
    InvalidFolderName,
}

impl FormError {
    pub(crate) fn field_errors(&self) -> Vec<FormFieldError> {
        self.field()
            .map(|field| {
                vec![FormFieldError {
                    field: Cow::Borrowed(field),
                    message: Cow::Owned(self.to_string()),
                }]
            })
            .unwrap_or_default()
    }

    fn field(&self) -> Option<&'static str> {
        match self {
            Self::MissingFolderName | Self::InvalidFolderName => Some("name"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FormError;

    fn field_errors(error: &FormError) -> Vec<(String, String)> {
        error
            .field_errors()
            .into_iter()
            .map(|error| (error.field.to_string(), error.message.into_owned()))
            .collect()
    }

    #[test]
    fn folder_validation_messages_stay_in_forms_layer() {
        assert_eq!(
            field_errors(&FormError::MissingFolderName),
            vec![("name".to_string(), "Введите название папки.".to_string())]
        );
        assert_eq!(
            field_errors(&FormError::InvalidFolderName),
            vec![("name".to_string(), "Недопустимое имя папки.".to_string())]
        );
    }
}
