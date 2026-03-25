use std::net::TcpListener;

use actix_cors::Cors;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::{App, HttpServer, dev::Server, middleware, web};
use pushkind_common::middleware::RedirectUnauthorized;
use pushkind_common::models::config::CommonServerConfig;
use pushkind_common::routes::logout;

use crate::models::config::{AppConfig, Settings};
use crate::routes::api::{api_v1_files_entries, api_v1_iam, api_v1_no_access};
use crate::routes::aux::not_assigned;
use crate::routes::main::{create_folder, index, upload_files};

pub mod domain;
pub mod dto;
pub mod forms;
pub mod frontend;
pub mod models;
pub mod routes;
pub mod services;

pub const SERVICE_ACCESS_ROLE: &str = "files";

/// Builds and runs the Actix-Web HTTP server using the provided configuration.
pub async fn run(settings: Settings) -> std::io::Result<()> {
    let bind_address = (settings.server.address.clone(), settings.server.port);
    let listener = TcpListener::bind(bind_address)?;

    build_server(listener, settings.app)?.await
}

/// Builds an Actix-Web HTTP server on a pre-bound listener.
pub fn build_server(listener: TcpListener, app_config: AppConfig) -> std::io::Result<Server> {
    let common_config = CommonServerConfig {
        auth_service_url: app_config.auth_service_url.to_string(),
        secret: app_config.secret.clone(),
    };

    // Keys and stores for identity and sessions.
    let secret_key = Key::from(app_config.secret.as_bytes());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // set to true in prod
                    .cookie_domain(Some(format!(".{}", app_config.domain)))
                    .build(),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(Files::new("/upload", &app_config.upload_path).show_files_listing())
            .service(Files::new("/assets", "./assets").prefer_utf8(true))
            .service(not_assigned)
            .service(
                web::scope("/api")
                    .service(api_v1_iam)
                    .service(api_v1_no_access)
                    .service(api_v1_files_entries),
            )
            .service(
                web::scope("")
                    .wrap(RedirectUnauthorized)
                    .service(index)
                    .service(logout)
                    .service(upload_files)
                    .service(create_folder),
            )
            .app_data(web::Data::new(app_config.clone()))
            .app_data(web::Data::new(common_config.clone()))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
