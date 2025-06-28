use std::env;

use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::{App, HttpServer, middleware, web};
use actix_web_flash_messages::{FlashMessagesFramework, storage::CookieMessageStore};
use dotenvy::dotenv;
use log::error;

use pushkind_files::middleware::RedirectUnauthorized;
use pushkind_files::models::config::ServerConfig;
use pushkind_files::routes::main::{index, logout, not_assigned};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    dotenv().ok(); // Load .env file

    let port = env::var("PORT").unwrap_or("8080".to_string());
    let port = port.parse::<u16>().unwrap_or(8080);
    let address = env::var("ADDRESS").unwrap_or("127.0.0.1".to_string());

    let secret = env::var("SECRET_KEY");
    let secret_key = match &secret {
        Ok(key) => Key::from(key.as_bytes()),
        Err(_) => Key::generate(),
    };

    let auth_service_url = env::var("AUTH_SERVICE_URL");
    let auth_service_url = match auth_service_url {
        Ok(auth_service_url) => auth_service_url,
        Err(_) => {
            error!("AUTH_SERVICE_URL environment variable not set");
            std::process::exit(1);
        }
    };

    let server_config = ServerConfig {
        secret: secret.unwrap_or_default(),
        auth_service_url,
    };

    let domain = env::var("DOMAIN").unwrap_or("localhost".to_string());

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // set to true in prod
                    .cookie_domain(Some(format!(".{domain}")))
                    .build(),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(Files::new("/upload", "./upload"))
            .service(Files::new("/assets", "./assets"))
            .service(not_assigned)
            .service(
                web::scope("")
                    .wrap(RedirectUnauthorized)
                    .service(index)
                    .service(logout),
            )
            .app_data(web::Data::new(server_config.clone()))
    })
    .bind((address, port))?
    .run()
    .await
}
