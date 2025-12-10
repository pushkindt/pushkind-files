use std::path::{Path, PathBuf};

use actix_cors::Cors;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::{App, HttpServer, middleware, web};
use actix_web_flash_messages::{FlashMessagesFramework, storage::CookieMessageStore};
use pushkind_common::middleware::RedirectUnauthorized;
use pushkind_common::models::config::CommonServerConfig;
use pushkind_common::routes::{logout, not_assigned};
use tera::Tera;

use crate::models::config::ServerConfig;
use crate::routes::main::{create_folder, index, upload_files};

pub mod forms;
pub mod models;
pub mod routes;

pub const SERVICE_ACCESS_ROLE: &str = "crm";

/// Returns `None` if the path is invalid (e.g., contains `..`)
/// Trims leading slashes
fn sanitize_path(input: &str) -> Option<PathBuf> {
    let trimmed = input.trim_start_matches('/');
    let path = Path::new(trimmed);

    // Reject paths with components that go up the directory tree
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return None;
    }

    Some(path.to_path_buf())
}

/// Sanitizes a file name ensuring it does not contain path separators,
/// parent directory components, or special entries like `.`.
fn sanitize_file_name(input: &str) -> Option<PathBuf> {
    let path = sanitize_path(input)?;
    let mut components = path.components();
    match (components.next(), components.next()) {
        // Only accept a single "normal" component such as `file.txt`
        (Some(std::path::Component::Normal(_)), None) => Some(path),
        _ => None,
    }
}

/// Returns `true` if the provided file name has a common image extension.
fn is_image_file(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "svg"
            )
        })
        .unwrap_or(false)
}

/// Builds and runs the Actix-Web HTTP server using the provided configuration.
pub async fn run(server_config: ServerConfig) -> std::io::Result<()> {
    let common_config = CommonServerConfig {
        auth_service_url: server_config.auth_service_url.to_string(),
        secret: server_config.secret.clone(),
    };

    // Keys and stores for identity, sessions, and flash messages.
    let secret_key = Key::from(server_config.secret.as_bytes());

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let tera = Tera::new(&server_config.templates_dir)
        .map_err(|e| std::io::Error::other(format!("Template parsing error(s): {e}")))?;

    let bind_address = (server_config.address.clone(), server_config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(message_framework.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // set to true in prod
                    .cookie_domain(Some(format!(".{}", server_config.domain)))
                    .build(),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(Files::new("/upload", &server_config.upload_path).show_files_listing())
            .service(Files::new("/assets", "./assets"))
            .service(not_assigned)
            .service(
                web::scope("")
                    .wrap(RedirectUnauthorized)
                    .service(index)
                    .service(logout)
                    .service(upload_files)
                    .service(create_folder),
            )
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(server_config.clone()))
            .app_data(web::Data::new(common_config.clone()))
    })
    .bind(bind_address)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn sanitize_path_valid() {
        let path = sanitize_path("folder/sub").expect("valid path");
        assert_eq!(path, Path::new("folder/sub"));
    }

    #[test]
    fn sanitize_path_invalid_parent() {
        assert!(sanitize_path("../secret").is_none());
        assert!(sanitize_path("folder/../secret").is_none());
    }

    #[test]
    fn sanitize_path_leading_slash() {
        let path = sanitize_path("/leading/path").expect("valid path");
        assert_eq!(path, Path::new("leading/path"));
    }

    #[test]
    fn sanitize_file_name_single_component() {
        let path = sanitize_file_name("image.png").expect("valid file name");
        assert_eq!(path, Path::new("image.png"));
    }

    #[test]
    fn sanitize_file_name_reject_nested() {
        assert!(sanitize_file_name("../secret.txt").is_none());
        assert!(sanitize_file_name("foo/bar.txt").is_none());
    }

    #[test]
    fn sanitize_file_name_reject_dot() {
        assert!(sanitize_file_name(".").is_none());
    }

    #[test]
    fn is_image_file_positive() {
        assert!(is_image_file("photo.JPG"));
        assert!(is_image_file("image.png"));
    }

    #[test]
    fn is_image_file_negative() {
        assert!(!is_image_file("document.txt"));
        assert!(!is_image_file("noextension"));
    }
}
