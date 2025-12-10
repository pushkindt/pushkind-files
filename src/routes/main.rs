use std::path::Path;

use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, get, post, web};
use actix_web_flash_messages::IncomingFlashMessages;
use pushkind_common::domain::auth::AuthenticatedUser;
use pushkind_common::models::config::CommonServerConfig;
use pushkind_common::routes::redirect;
use pushkind_common::routes::{base_context, render_template};
use serde::Deserialize;
use tera::Tera;

use crate::domain::UploadRoot;
use crate::dto::FileEntryDto;
use crate::forms::main::{CreateFolderForm, UploadFileForm};
use crate::models::config::ServerConfig;
use crate::services::ServiceError;
use crate::services::files::FileService;

/// Query parameters for the [`index`] route.
#[derive(Deserialize)]
struct IndexQueryParams {
    /// Optional path relative to the user's upload directory.
    path: Option<String>,
}

fn file_service(server_config: &ServerConfig) -> FileService {
    FileService::new(UploadRoot::from(
        Path::new(&server_config.upload_path).to_path_buf(),
    ))
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
    let mut context = base_context(
        &flash_messages,
        &user,
        "index",
        &common_config.auth_service_url,
    );

    let service = file_service(&server_config);

    let entries: Vec<FileEntryDto> = match service.list_entries(&user, params.path.as_deref()) {
        Ok(entries) => entries,
        Err(ServiceError::Unauthorized) => return redirect("/na"),
        Err(ServiceError::InvalidPath) => return HttpResponse::BadRequest().body("Invalid path"),
        Err(e) => {
            log::error!("Failed to list entries: {e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    context.insert("entries", &entries);
    context.insert("path", &params.path.clone().unwrap_or_default());

    render_template(&tera, "main/index.html", &context)
}

/// Render the file browser fragment for embedding in other pages or services.
#[get("/files/browser")]
pub async fn file_browser(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
    common_config: web::Data<CommonServerConfig>,
    server_config: web::Data<ServerConfig>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let mut context = base_context(
        &flash_messages,
        &user,
        "file_browser",
        &common_config.auth_service_url,
    );

    let service = file_service(&server_config);

    let entries: Vec<FileEntryDto> = match service.list_entries(&user, params.path.as_deref()) {
        Ok(entries) => entries,
        Err(ServiceError::Unauthorized) => return redirect("/na"),
        Err(ServiceError::InvalidPath) => return HttpResponse::BadRequest().body("Invalid path"),
        Err(e) => {
            log::error!("Failed to list entries: {e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    context.insert("entries", &entries);
    context.insert("path", &params.path.clone().unwrap_or_default());

    render_template(&tera, "components/file_browser.html", &context)
}

/// Handle a file upload and save it to the user's directory.
#[post("/files/upload")]
pub async fn upload_files(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    MultipartForm(form): MultipartForm<UploadFileForm>,
    server_config: web::Data<ServerConfig>,
) -> impl Responder {
    let temp_file = form.file;

    let service = file_service(&server_config);

    match service.persist_upload(
        &user,
        params.path.as_deref(),
        temp_file.file_name.clone(),
        temp_file,
    ) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(ServiceError::InvalidFileName) | Err(ServiceError::InvalidPath) => {
            HttpResponse::BadRequest().body("Некорректный файл или путь для загрузки.")
        }
        Err(ServiceError::Unauthorized) => HttpResponse::Unauthorized().finish(),
        Err(e) => {
            log::error!("File upload error: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Create a new folder in the user's upload directory.
#[post("/folder/create")]
pub async fn create_folder(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    web::Form(form): web::Form<CreateFolderForm>,
    server_config: web::Data<ServerConfig>,
) -> impl Responder {
    let service = file_service(&server_config);

    match service.create_folder(&user, params.path.as_deref(), &form) {
        Ok(()) => HttpResponse::Created().finish(),
        Err(ServiceError::Validation(msg)) => HttpResponse::BadRequest().body(msg),
        Err(ServiceError::InvalidPath) => {
            HttpResponse::BadRequest().body("Недопустимый путь для загрузки файла.")
        }
        Err(ServiceError::Unauthorized) => HttpResponse::Unauthorized().body("Недостаточно прав."),
        Err(e) => {
            log::error!("Failed to create upload directory: {e:?}");
            HttpResponse::InternalServerError().body("Не удалось создать папку")
        }
    }
}
