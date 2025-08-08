use std::fs;
use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{
    HttpResponse, Responder,
    error::{ErrorBadRequest, ErrorUnauthorized},
    get, post, web,
};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use pushkind_common::models::auth::AuthenticatedUser;
use pushkind_common::models::config::CommonServerConfig;
use pushkind_common::routes::{alert_level_to_str, ensure_role, redirect};
use serde::Deserialize;
use tera::Context;
use uuid::Uuid;
use validator::Validate;

use crate::forms::main::{CreateFolderForm, UploadFileForm};
use crate::routes::render_template;
use crate::{is_image_file, sanitize_file_name, sanitize_path};

#[derive(Deserialize)]
struct IndexQueryParams {
    path: Option<String>,
}

#[derive(serde::Serialize)]
struct FileEntry {
    name: String,
    is_directory: bool,
    is_image: bool,
}

#[get("/")]
pub async fn index(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
    server_config: web::Data<CommonServerConfig>,
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
        log::error!("Failed to create base path: {e:?}");
        return HttpResponse::InternalServerError().finish();
    }

    let target_path = base_path.join(&sanitized_path);
    if !target_path.starts_with(&base_path) {
        return HttpResponse::Unauthorized().finish();
    }

    // Read entries
    let mut entries: Vec<FileEntry> = match fs::read_dir(&target_path) {
        Ok(read_dir) => read_dir
            .filter_map(|e| e.ok())
            .map(|entry| {
                let file_type = entry.file_type().ok();
                let is_directory = file_type.map(|ft| ft.is_dir()).unwrap_or(false);
                let name = entry.file_name().to_string_lossy().to_string();
                let is_image = !is_directory && is_image_file(&name);
                FileEntry {
                    name,
                    is_directory,
                    is_image,
                }
            })
            .collect(),
        Err(err) => {
            log::warn!("Cannot read dir: {target_path:?}: {err}");
            vec![]
        }
    };
    entries.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less, // Folders before files
            (false, true) => std::cmp::Ordering::Greater, // Files after folders
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()), // Alphabetical
        }
    });

    context.insert("entries", &entries);
    context.insert("path", &sanitized_path);

    render_template("main/index.html", &context)
}

#[get("/na")]
pub async fn not_assigned(
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
    server_config: web::Data<CommonServerConfig>,
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

#[get("/upload/{hub_id}/{path:.*}")]
pub async fn download_file(
    params: web::Path<(i32, String)>,
    user: AuthenticatedUser,
) -> actix_web::Result<NamedFile> {
    let (hub_id, tail) = params.into_inner();
    if hub_id != user.hub_id {
        return Err(ErrorUnauthorized("invalid hub"));
    }
    let sanitized = sanitize_path(&tail).ok_or(ErrorBadRequest("Invalid path"))?;
    let base_path = Path::new(crate::UPLOAD_PATH).join(user.hub_id.to_string());
    let file_path = base_path.join(sanitized);
    if !file_path.starts_with(&base_path) {
        return Err(ErrorUnauthorized("invalid hub"));
    }
    Ok(NamedFile::open(file_path)?)
}

#[post("/files/upload")]
pub async fn upload_files(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    MultipartForm(form): MultipartForm<UploadFileForm>,
) -> impl Responder {
    if let Err(_) = ensure_role(&user, "files", None) {
        return HttpResponse::Unauthorized().finish();
    }

    let raw_file_name = form
        .file
        .file_name
        .unwrap_or_else(|| format!("upload-{}", Uuid::new_v4()));

    let file_name = match sanitize_file_name(&raw_file_name) {
        Some(p) => p,
        None => {
            return HttpResponse::BadRequest().body("Incorrect file name");
        }
    };

    // Base directory: ./upload/{hub_id}
    let base_path = Path::new(crate::UPLOAD_PATH).join(user.hub_id.to_string());

    // Sanitize path parameter to prevent directory traversal
    let sanitized_path = match params.path.as_deref() {
        Some(p) => match sanitize_path(p) {
            Some(p) => p,
            None => {
                return HttpResponse::BadRequest().body("Incorrect path");
            }
        },
        None => PathBuf::new(),
    };

    // Final upload directory
    let target_dir = base_path.join(sanitized_path);
    if !target_dir.starts_with(&base_path) {
        return HttpResponse::Unauthorized().finish();
    }

    if let Err(e) = std::fs::create_dir_all(&target_dir) {
        log::error!("Failed to create upload directory: {e:?}");
        return HttpResponse::InternalServerError().finish();
    }

    // Save file to path
    let filepath = target_dir.join(file_name);

    match form.file.file.persist(filepath) {
        Ok(_) => FlashMessage::success("Файл успешно загружен.").send(),
        Err(e) => {
            log::error!("File upload error: {e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Ok().finish()
}

#[post("/folder/create")]
pub async fn create_folder(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    web::Form(form): web::Form<CreateFolderForm>,
) -> impl Responder {
    if let Err(response) = ensure_role(&user, "files", Some("/na")) {
        return response;
    }

    if let Err(e) = form.validate() {
        log::error!("Validation error: {e:?}");
        FlashMessage::error("Ошибка валидации формы.").send();
        return redirect(&format!(
            "/?path={}",
            params.path.clone().unwrap_or_default()
        ));
    }

    // Base directory: ./upload/{hub_id}
    let base_path = Path::new(crate::UPLOAD_PATH).join(user.hub_id.to_string());

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
    if !target_dir.starts_with(&base_path) {
        return HttpResponse::Unauthorized().finish();
    }

    if let Err(e) = std::fs::create_dir_all(&target_dir) {
        log::error!("Failed to create upload directory: {e:?}");
        return HttpResponse::InternalServerError().finish();
    }

    FlashMessage::success("Папка успешно создана.").send();

    redirect(&format!(
        "/?path={}",
        params.path.clone().unwrap_or_default()
    ))
}
