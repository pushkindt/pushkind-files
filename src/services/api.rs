use pushkind_common::domain::auth::AuthenticatedUser;
use pushkind_common::dto::shell::{CurrentUserDto, IamDto, NoAccessPageDto};
use pushkind_common::models::config::CommonServerConfig;

use crate::models::config::AppConfig;
use crate::services::ServiceResult;
use crate::services::files::FileService;

pub fn get_shell_data(
    user: &AuthenticatedUser,
    common_config: &CommonServerConfig,
    app_config: &AppConfig,
) -> ServiceResult<IamDto> {
    let service = FileService::from_app_config(app_config);
    service.validate_browser_access(user, None)?;

    Ok(IamDto {
        current_user: CurrentUserDto::from(user.clone()),
        home_url: common_config.auth_service_url.clone(),
        navigation: Vec::new(),
        local_menu_items: Vec::new(),
        hub_name: "Files".to_string(),
    })
}

pub fn get_file_entries_data(
    path: Option<&str>,
    user: &AuthenticatedUser,
    app_config: &AppConfig,
) -> ServiceResult<crate::dto::FileBrowserDataDto> {
    let service = FileService::from_app_config(app_config);
    service.list_browser_data(user, path)
}

pub fn get_no_access_data(
    user: &AuthenticatedUser,
    common_config: &CommonServerConfig,
) -> NoAccessPageDto {
    NoAccessPageDto {
        current_user: CurrentUserDto::from(user.clone()),
        home_url: common_config.auth_service_url.clone(),
        required_role: Some(crate::SERVICE_ACCESS_ROLE.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::ServiceError;
    use tempfile::tempdir;

    fn test_user() -> AuthenticatedUser {
        AuthenticatedUser {
            sub: "user".into(),
            email: "user@example.com".into(),
            hub_id: 42,
            name: "User".into(),
            roles: vec![crate::SERVICE_ACCESS_ROLE.to_string()],
            exp: 0,
        }
    }

    fn unauthorized_user() -> AuthenticatedUser {
        AuthenticatedUser {
            roles: vec![],
            ..test_user()
        }
    }

    fn app_config(upload_path: String) -> AppConfig {
        AppConfig {
            domain: "example.com".into(),
            auth_service_url: "https://auth.example.com".into(),
            secret: "supersecret".repeat(8),
            upload_path,
        }
    }

    fn common_config() -> CommonServerConfig {
        CommonServerConfig {
            auth_service_url: "https://auth.example.com".into(),
            secret: "supersecret".repeat(8),
        }
    }

    #[test]
    fn shell_data_returns_context() {
        let response = get_shell_data(
            &test_user(),
            &common_config(),
            &app_config("./upload".into()),
        );
        assert!(response.is_ok());
    }

    #[test]
    fn shell_data_rejects_missing_role() {
        let response = get_shell_data(
            &unauthorized_user(),
            &common_config(),
            &app_config("./upload".into()),
        );

        assert!(matches!(response, Err(ServiceError::Unauthorized)));
    }

    #[test]
    fn no_access_data_does_not_require_role() {
        let response = get_no_access_data(&unauthorized_user(), &common_config());
        assert_eq!(response.current_user.email, "user@example.com");
    }

    #[test]
    fn file_entries_returns_directory_data() {
        let dir = tempdir().unwrap();
        let hub_root = dir.path().join("42");
        std::fs::create_dir_all(&hub_root).unwrap();
        std::fs::write(hub_root.join("poster.png"), b"png").unwrap();

        let response = get_file_entries_data(
            None,
            &test_user(),
            &app_config(dir.path().to_string_lossy().into_owned()),
        );

        assert!(response.is_ok());
    }

    #[test]
    fn file_entries_rejects_invalid_path() {
        let response = get_file_entries_data(
            Some("../evil"),
            &test_user(),
            &app_config("./upload".into()),
        );

        assert!(matches!(response, Err(ServiceError::InvalidPath)));
    }

    #[test]
    fn file_entries_rejects_missing_role() {
        let response =
            get_file_entries_data(None, &unauthorized_user(), &app_config("./upload".into()));

        assert!(matches!(response, Err(ServiceError::Unauthorized)));
    }
}
