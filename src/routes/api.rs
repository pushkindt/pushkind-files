use actix_web::{HttpResponse, get, web};
use log::error;
use pushkind_common::domain::auth::AuthenticatedUser;
use pushkind_common::models::config::CommonServerConfig;
use serde::Deserialize;

use crate::models::config::AppConfig;
use crate::services::ServiceError;
use crate::services::api as api_service;

#[derive(Deserialize)]
pub struct EntriesQueryParams {
    path: Option<String>,
}

#[get("/v1/iam")]
pub async fn api_v1_iam(
    user: AuthenticatedUser,
    common_config: web::Data<CommonServerConfig>,
) -> HttpResponse {
    match api_service::get_shell_data(&user, &common_config) {
        Ok(shell) => HttpResponse::Ok().json(shell),
        Err(error) => {
            error!("Failed to build shell context: {error:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/v1/files/entries")]
pub async fn api_v1_files_entries(
    params: web::Query<EntriesQueryParams>,
    user: AuthenticatedUser,
    app_config: web::Data<AppConfig>,
) -> HttpResponse {
    match api_service::get_file_entries_data(params.path.as_deref(), &user, &app_config) {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(ServiceError::InvalidPath) => HttpResponse::BadRequest().finish(),
        Err(ServiceError::Unauthorized) => HttpResponse::Unauthorized().finish(),
        Err(error) => {
            error!("Failed to list browser API entries: {error:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/v1/no-access")]
pub async fn api_v1_no_access(
    user: AuthenticatedUser,
    common_config: web::Data<CommonServerConfig>,
) -> HttpResponse {
    HttpResponse::Ok().json(api_service::get_no_access_data(&user, &common_config))
}
