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
use crate::routes::main::{create_folder, file_browser, index, upload_files};

pub mod domain;
pub mod dto;
pub mod forms;
pub mod models;
pub mod routes;
pub mod services;

pub const SERVICE_ACCESS_ROLE: &str = "files";

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
                    .service(file_browser)
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
mod tests {}
