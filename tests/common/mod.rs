#![allow(dead_code)]

use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::Once;
use std::time::Duration;

use actix_cors::Cors;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::rt::time::sleep;
use actix_web::{
    App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, middleware, post, web,
};
use pushkind_common::domain::auth::AuthenticatedUser;
use pushkind_common::middleware::RedirectUnauthorized;
use pushkind_common::models::config::CommonServerConfig;
use pushkind_common::routes::logout;
use reqwest::{Client, StatusCode, redirect::Policy};
use tempfile::TempDir;

use pushkind_files::models::config::AppConfig;
use pushkind_files::routes::api::{api_v1_files_entries, api_v1_iam, api_v1_no_access};
use pushkind_files::routes::aux::not_assigned;
use pushkind_files::routes::main::{create_folder, index, upload_files};

pub const HUB_ID: i32 = 42;

static FRONTEND_ASSETS: Once = Once::new();

pub struct TestApp {
    _upload_dir: TempDir,
    address: String,
    upload_root: PathBuf,
}

impl TestApp {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn upload_root(&self) -> &Path {
        &self.upload_root
    }
}

#[derive(serde::Deserialize)]
struct LoginRequest {
    hub_id: i32,
    email: String,
    name: String,
    roles: Vec<String>,
}

#[post("/test/login")]
async fn test_login(
    request: HttpRequest,
    payload: web::Json<LoginRequest>,
    common_config: web::Data<CommonServerConfig>,
) -> impl Responder {
    let mut user = AuthenticatedUser {
        sub: payload.email.clone(),
        email: payload.email.clone(),
        hub_id: payload.hub_id,
        name: payload.name.clone(),
        roles: payload.roles.clone(),
        exp: 0,
    };
    user.set_expiration(7);

    let token = user
        .to_jwt(&common_config.secret)
        .expect("JWT generation should succeed for test users.");
    Identity::login(&request.extensions(), token).expect("Test login should persist identity.");

    HttpResponse::Ok().finish()
}

async fn wait_until_server_is_ready(address: &str) {
    let client = Client::builder()
        .redirect(Policy::none())
        .timeout(Duration::from_millis(100))
        .build()
        .expect("Failed to create the test HTTP client.");
    let url = format!("{address}/");

    for _ in 0..20 {
        match client.get(&url).send().await {
            Ok(response)
                if response.status() == StatusCode::SEE_OTHER
                    || response.status() == StatusCode::OK =>
            {
                return;
            }
            Ok(_) | Err(_) => sleep(Duration::from_millis(25)).await,
        }
    }

    panic!("Test server did not become ready at {url}");
}

fn ensure_test_frontend_assets() {
    FRONTEND_ASSETS.call_once(|| {
        let fixtures = [
            (
                "assets/dist/app/index.html",
                "<!doctype html><html><head><title>Files</title></head><body>files-page</body></html>",
            ),
            (
                "assets/dist/app/no-access.html",
                "<!doctype html><html><head><title>Files No Access</title></head><body>files-page files-no-access</body></html>",
            ),
        ];

        for (path, contents) in fixtures {
            let path = Path::new(path);
            if path.exists() {
                continue;
            }

            let parent = path
                .parent()
                .expect("frontend fixture path should include a parent directory");
            fs::create_dir_all(parent).expect("failed to create frontend fixture directory");
            fs::write(path, contents).expect("failed to write frontend fixture file");
        }
    });
}

pub async fn spawn_app() -> TestApp {
    ensure_test_frontend_assets();

    let upload_dir = TempDir::new().expect("Failed to create upload temp dir.");
    let upload_root = upload_dir.path().join("upload");
    std::fs::create_dir_all(&upload_root).expect("Upload root should be creatable.");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random local port.");
    let port = listener
        .local_addr()
        .expect("Failed to read the local socket address.")
        .port();

    let app_config = AppConfig {
        domain: "localhost".to_string(),
        auth_service_url: "https://users.pushkind.test/auth/signin".to_string(),
        secret: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
        upload_path: upload_root.to_string_lossy().into_owned(),
    };
    let common_config = CommonServerConfig {
        auth_service_url: app_config.auth_service_url.clone(),
        secret: app_config.secret.clone(),
    };
    let secret_key = Key::from(app_config.secret.as_bytes());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                actix_files::Files::new("/upload", &app_config.upload_path).show_files_listing(),
            )
            .service(actix_files::Files::new("/assets", "./assets").prefer_utf8(true))
            .service(test_login)
            .service(not_assigned)
            .service(
                web::scope("/api")
                    .service(api_v1_iam)
                    .service(api_v1_no_access)
                    .service(api_v1_files_entries),
            )
            .service(upload_files)
            .service(create_folder)
            .service(
                web::scope("")
                    .wrap(RedirectUnauthorized)
                    .service(index)
                    .service(logout),
            )
            .app_data(web::Data::new(app_config.clone()))
            .app_data(web::Data::new(common_config.clone()))
    })
    .listen(listener)
    .expect("Failed to listen with the test server.")
    .run();

    actix_web::rt::spawn(server);
    let address = format!("http://127.0.0.1:{port}");

    wait_until_server_is_ready(&address).await;

    TestApp {
        _upload_dir: upload_dir,
        address,
        upload_root,
    }
}

pub fn build_reqwest_client() -> Client {
    Client::builder()
        .cookie_store(true)
        .build()
        .expect("Can't create a request client")
}

pub fn build_no_redirect_client() -> Client {
    Client::builder()
        .cookie_store(true)
        .redirect(Policy::none())
        .build()
        .expect("Can't create a request client")
}

pub async fn login_as(
    client: &Client,
    address: &str,
    email: &str,
    name: &str,
    hub_id: i32,
    roles: &[&str],
) {
    let response = client
        .post(format!("{address}/test/login"))
        .json(&serde_json::json!({
            "hub_id": hub_id,
            "email": email,
            "name": name,
            "roles": roles,
        }))
        .send()
        .await
        .expect("Failed to submit test login.");

    assert_eq!(response.status(), StatusCode::OK);
}
