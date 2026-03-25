use actix_multipart::form::MultipartForm;
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use pushkind_common::domain::auth::AuthenticatedUser;
use pushkind_common::routes::redirect;
use serde::Deserialize;

use crate::dto::{ApiMutationErrorDto, ApiMutationSuccessDto};
use crate::forms::main::{CreateFolderForm, CreateFolderPayload, UploadFileForm};
use crate::frontend::open_frontend_html;
use crate::models::config::AppConfig;
use crate::services::ServiceError;
use crate::services::files::FileService;

/// Query parameters for the [`index`] route.
#[derive(Deserialize)]
struct IndexQueryParams {
    /// Optional path relative to the user's upload directory.
    path: Option<String>,
}

fn mutation_success_response(status: actix_web::http::StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(ApiMutationSuccessDto {
        message: message.to_string(),
        redirect_to: None,
    })
}

fn mutation_error_response(
    status: actix_web::http::StatusCode,
    message: &str,
    field_errors: Vec<crate::dto::ApiFieldErrorDto>,
) -> HttpResponse {
    HttpResponse::build(status).json(ApiMutationErrorDto {
        message: message.to_string(),
        field_errors,
    })
}

fn upload_response(result: Result<(), ServiceError>) -> HttpResponse {
    match result {
        Ok(()) => {
            mutation_success_response(actix_web::http::StatusCode::OK, "Файл успешно загружен.")
        }
        Err(ServiceError::InvalidFileName) | Err(ServiceError::InvalidPath) => {
            mutation_error_response(
                actix_web::http::StatusCode::BAD_REQUEST,
                "Некорректный файл или путь для загрузки.",
                Vec::new(),
            )
        }
        Err(ServiceError::Unauthorized) => mutation_error_response(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Недостаточно прав для загрузки файлов.",
            Vec::new(),
        ),
        Err(error) => {
            log::error!("File upload error: {error:?}");
            mutation_error_response(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Не удалось загрузить файл.",
                Vec::new(),
            )
        }
    }
}

fn create_folder_response(result: Result<(), ServiceError>) -> HttpResponse {
    match result {
        Ok(()) => mutation_success_response(
            actix_web::http::StatusCode::CREATED,
            "Папка успешно создана.",
        ),
        Err(ServiceError::Form(error)) => {
            HttpResponse::BadRequest().json(ApiMutationErrorDto::from(&error))
        }
        Err(ServiceError::InvalidPath) => mutation_error_response(
            actix_web::http::StatusCode::BAD_REQUEST,
            "Недопустимый путь для создания папки.",
            Vec::new(),
        ),
        Err(ServiceError::Unauthorized) => mutation_error_response(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Недостаточно прав.",
            Vec::new(),
        ),
        Err(error) => {
            log::error!("Failed to create upload directory: {error:?}");
            mutation_error_response(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Не удалось создать папку.",
                Vec::new(),
            )
        }
    }
}

/// Serve the React-owned files page document for the authenticated user.
#[get("/")]
pub async fn index(
    request: HttpRequest,
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    app_config: web::Data<AppConfig>,
) -> impl Responder {
    let service = FileService::from_app_config(&app_config);
    match service.validate_browser_access(&user, params.path.as_deref()) {
        Ok(()) => {}
        Err(ServiceError::Unauthorized) => return redirect("/na"),
        Err(ServiceError::InvalidPath) => {
            return HttpResponse::BadRequest().body("Invalid path");
        }
        Err(e) => {
            log::error!("Failed to validate file browser access: {e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    }

    match open_frontend_html("assets/dist/app/index.html").await {
        Ok(file) => file.into_response(&request),
        Err(error) => {
            log::error!("Failed to open files frontend document: {error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Serve the React-owned embedded browser document for same-origin consumers.
#[get("/files/browser")]
pub async fn file_browser(
    request: HttpRequest,
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    app_config: web::Data<AppConfig>,
) -> impl Responder {
    let service = FileService::from_app_config(&app_config);
    match service.validate_browser_access(&user, params.path.as_deref()) {
        Ok(()) => {}
        Err(ServiceError::Unauthorized) => return redirect("/na"),
        Err(ServiceError::InvalidPath) => {
            return HttpResponse::BadRequest().body("Invalid path");
        }
        Err(error) => {
            log::error!("Failed to validate embedded file browser access: {error:?}");
            return HttpResponse::InternalServerError().finish();
        }
    }

    match open_frontend_html("assets/dist/app/browser.html").await {
        Ok(file) => file.into_response(&request),
        Err(error) => {
            log::error!("Failed to open embedded browser frontend document: {error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Handle a file upload and save it to the user's directory.
#[post("/files/upload")]
pub async fn upload_files(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    MultipartForm(form): MultipartForm<UploadFileForm>,
    app_config: web::Data<AppConfig>,
) -> impl Responder {
    let temp_file = form.file;

    let service = FileService::from_app_config(&app_config);

    upload_response(service.persist_upload(
        &user,
        params.path.as_deref(),
        temp_file.file_name.clone(),
        temp_file,
    ))
}

/// Create a new folder in the user's upload directory.
#[post("/folder/create")]
pub async fn create_folder(
    params: web::Query<IndexQueryParams>,
    user: AuthenticatedUser,
    web::Form(form): web::Form<CreateFolderForm>,
    app_config: web::Data<AppConfig>,
) -> impl Responder {
    let service = FileService::from_app_config(&app_config);
    let payload = match CreateFolderPayload::try_from(form) {
        Ok(payload) => payload,
        Err(error) => return HttpResponse::BadRequest().json(ApiMutationErrorDto::from(&error)),
    };

    create_folder_response(service.create_folder(&user, params.path.as_deref(), payload))
}

#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;

    use super::*;
    use crate::forms::FormError;

    #[actix_web::test]
    async fn create_folder_json_validation_response_includes_field_errors() {
        let response =
            create_folder_response(Err(ServiceError::Form(FormError::MissingFolderName)));

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body()).await.unwrap();
        let payload: ApiMutationErrorDto = serde_json::from_slice(&body).unwrap();
        assert_eq!(payload.message, "Ошибка валидации формы.");
        assert_eq!(payload.field_errors[0].field, "name");
        assert_eq!(payload.field_errors[0].message, "Введите название папки.");
    }

    #[actix_web::test]
    async fn upload_json_invalid_path_response_is_structured() {
        let response = upload_response(Err(ServiceError::InvalidPath));

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body()).await.unwrap();
        let payload: ApiMutationErrorDto = serde_json::from_slice(&body).unwrap();
        assert_eq!(payload.message, "Некорректный файл или путь для загрузки.");
        assert!(payload.field_errors.is_empty());
    }

    #[actix_web::test]
    async fn create_folder_validation_response_is_json_only() {
        let response =
            create_folder_response(Err(ServiceError::Form(FormError::MissingFolderName)));

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
