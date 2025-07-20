use std::fs;
use std::path::{Path, PathBuf};

use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, get, post, web};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use serde::Deserialize;
use tera::Context;
use uuid::Uuid;

use crate::forms::main::{CreateFolderForm, UploadFileForm};
use crate::models::auth::AuthenticatedUser;
use crate::models::config::ServerConfig;
use crate::routes::{alert_level_to_str, ensure_role, redirect, render_template};
use crate::sanitize_path;

#[derive(Deserialize)]
struct IndexQueryParams {
    path: Option<String>,
}

#[derive(serde::Serialize)]
struct FileEntry {
    name: String,
    is_directory: bool,
}

#[get("/")]
pub async fn index(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
    server_config: web::Data<ServerConfig>,
) -> impl Responder {
    if let Err(response) = ensure_role(&user, "files", Some("/na")) {
        return response;
    }

    let alerts: Vec<_> = flash_messages
        .iter()
        .map(|f| (f.content(), alert_level_to_str(&f.level())))
        .collect();

    let mut context = Context::new();
    context.insert("alerts", &alerts);
    context.insert("current_user", &user);
    context.insert("current_page", "index");
    context.insert("home_url", &server_config.auth_service_url);

    // Sanitize and validate the path
    let path_param = params.path.as_deref().unwrap_or("");
    let sanitized_path = match sanitize_path(path_param) {
        Some(sanitized_path) => sanitized_path,
        None => return HttpResponse::BadRequest().body("Invalid path"),
    };

    // Construct the full path to the hub directory
    let base_path = Path::new(crate::UPLOAD_PATH).join(user.hub_id.to_string());
    if let Err(e) = fs::create_dir_all(&base_path) {
        log::error!("Failed to create base path: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    let target_path = base_path.join(&sanitized_path);

    // Read entries
    let mut entries: Vec<FileEntry> = match fs::read_dir(&target_path) {
        Ok(read_dir) => read_dir
            .filter_map(|e| e.ok())
            .map(|entry| {
                let file_type = entry.file_type().ok();
                let is_directory = file_type.map(|ft| ft.is_dir()).unwrap_or(false);
                let name = entry.file_name().to_string_lossy().to_string();
                FileEntry { name, is_directory }
            })
            .collect(),
        Err(err) => {
            log::warn!("Cannot read dir: {:?}: {}", target_path, err);
            vec![]
        }
    };
    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    context.insert("entries", &entries);
    context.insert("path", &sanitized_path);

    render_template("main/index.html", &context)
}

#[post("/logout")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    redirect("/")
}

#[get("/na")]
pub async fn not_assigned(
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
    server_config: web::Data<ServerConfig>,
) -> impl Responder {
    let alerts = flash_messages
        .iter()
        .map(|f| (f.content(), alert_level_to_str(&f.level())))
        .collect::<Vec<_>>();
    let mut context = Context::new();
    context.insert("alerts", &alerts);
    context.insert("current_user", &user);
    context.insert("current_page", "index");
    context.insert("home_url", &server_config.auth_service_url);

    render_template("main/not_assigned.html", &context)
}

#[post("/upload_image")]
pub async fn upload_image(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    MultipartForm(form): MultipartForm<UploadFileForm>,
) -> impl Responder {
    let file_name = form
        .image
        .file_name
        .unwrap_or_else(|| format!("upload-{}", Uuid::new_v4()));

    // Base directory: ./upload/{hub_id}
    let base_path = Path::new(crate::UPLOAD_PATH).join(&user.hub_id.to_string());

    // Sanitize path parameter to prevent directory traversal
    let sanitized_path = match params.path.as_deref() {
        Some(p) => match sanitize_path(p) {
            Some(p) => p,
            None => {
                FlashMessage::error("Недопустимый путь для загрузки файла.").send();
                return redirect("/");
            }
        },
        None => PathBuf::new(),
    };

    // Final upload directory
    let target_dir = base_path.join(sanitized_path);

    if let Err(e) = std::fs::create_dir_all(&target_dir) {
        log::error!("Failed to create upload directory: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    // Save file to path
    let filepath = target_dir.join(file_name);

    match form.image.file.persist(filepath) {
        Ok(_) => FlashMessage::success("Файл успешно загружен.").send(),
        Err(e) => {
            log::error!("File upload error: {:?}", e);
            FlashMessage::error("Ошибка при загрузке файла.").send();
        }
    }

    redirect(&format!(
        "/?path={}",
        params.path.clone().unwrap_or_default()
    ))
}

#[post("/folder/create")]
pub async fn create_folder(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    web::Form(form): web::Form<CreateFolderForm>,
) -> impl Responder {
    // Base directory: ./upload/{hub_id}
    let base_path = Path::new(crate::UPLOAD_PATH).join(&user.hub_id.to_string());

    // Sanitize path parameter to prevent directory traversal
    let sanitized_path = match params.path.as_deref() {
        Some(p) => match sanitize_path(p) {
            Some(p) => p,
            None => {
                FlashMessage::error("Недопустимый путь для загрузки файла.").send();
                return redirect("/");
            }
        },
        None => PathBuf::new(),
    };

    let new_path = match sanitize_path(&form.name) {
        Some(p) => p,
        None => {
            FlashMessage::error("Недопустимый путь для загрузки файла.").send();
            return redirect("/");
        }
    };

    // Final upload directory
    let target_dir = base_path.join(sanitized_path).join(new_path);

    if let Err(e) = std::fs::create_dir_all(&target_dir) {
        log::error!("Failed to create upload directory: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    FlashMessage::success("Папка успешно создана.").send();

    redirect(&format!(
        "/?path={}",
        params.path.clone().unwrap_or_default()
    ))
}
