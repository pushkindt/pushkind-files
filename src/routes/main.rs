use std::fs;
use std::path::{Path, PathBuf};

use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, get, post, web};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use pushkind_common::domain::auth::AuthenticatedUser;
use pushkind_common::models::config::CommonServerConfig;
use pushkind_common::routes::{base_context, render_template};
use pushkind_common::routes::{ensure_role, redirect};
use serde::Deserialize;
use tera::Tera;
use uuid::Uuid;
use validator::Validate;

use crate::forms::main::{CreateFolderForm, UploadFileForm};
use crate::models::config::ServerConfig;
use crate::{is_image_file, sanitize_file_name, sanitize_path};

/// Query parameters for the [`index`] route.
#[derive(Deserialize)]
struct IndexQueryParams {
    /// Optional path relative to the user's upload directory.
    path: Option<String>,
}

/// Representation of a single file system entry for template rendering.
#[derive(serde::Serialize)]
struct FileEntry {
    /// File or directory name.
    name: String,
    /// Whether the entry is a directory.
    is_directory: bool,
    /// Whether the entry is an image file.
    is_image: bool,
}

/// Display the contents of the current directory for the authenticated user.
#[get("/")]
pub async fn index(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
    common_config: web::Data<CommonServerConfig>,
    server_config: web::Data<ServerConfig>,
    tera: web::Data<Tera>,
) -> impl Responder {
    if let Err(response) = ensure_role(&user, "files", Some("/na")) {
        return response;
    }

    let mut context = base_context(
        &flash_messages,
        &user,
        "index",
        &common_config.auth_service_url,
    );

    // Sanitize and validate the path
    let path_param = params.path.as_deref().unwrap_or("");
    let sanitized_path = match sanitize_path(path_param) {
        Some(sanitized_path) => sanitized_path,
        None => return HttpResponse::BadRequest().body("Invalid path"),
    };

    // Construct the full path to the hub directory
    let base_path = Path::new(&server_config.upload_path).join(user.hub_id.to_string());
    if let Err(e) = fs::create_dir_all(&base_path) {
        log::error!("Failed to create base path: {e:?}");
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

    render_template(&tera, "main/index.html", &context)
}

/// Handle a file upload and save it to the user's directory.
#[post("/files/upload")]
pub async fn upload_files(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    MultipartForm(form): MultipartForm<UploadFileForm>,
    server_config: web::Data<ServerConfig>,
) -> impl Responder {
    if ensure_role(&user, "files", None).is_err() {
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
    let base_path = Path::new(&server_config.upload_path).join(user.hub_id.to_string());

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

/// Create a new folder in the user's upload directory.
#[post("/folder/create")]
pub async fn create_folder(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    web::Form(form): web::Form<CreateFolderForm>,
    server_config: web::Data<ServerConfig>,
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
    let base_path = Path::new(&server_config.upload_path).join(user.hub_id.to_string());

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
        log::error!("Failed to create upload directory: {e:?}");
        return HttpResponse::InternalServerError().finish();
    }

    FlashMessage::success("Папка успешно создана.").send();

    redirect(&format!(
        "/?path={}",
        params.path.clone().unwrap_or_default()
    ))
}
