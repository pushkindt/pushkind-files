use std::path::Path;

use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, Responder, get, post};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use tera::Context;
use uuid::Uuid;

use crate::forms::main::UploadFileForm;
use crate::models::auth::AuthenticatedUser;
use crate::routes::{alert_level_to_str, ensure_role, redirect, render_template};

#[get("/")]
pub async fn index(
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
) -> impl Responder {
    if let Err(response) = ensure_role(&user, "files", Some("/na")) {
        return response;
    };

    let alerts = flash_messages
        .iter()
        .map(|f| (f.content(), alert_level_to_str(&f.level())))
        .collect::<Vec<_>>();
    let mut context = Context::new();
    context.insert("alerts", &alerts);
    context.insert("current_user", &user);
    context.insert("current_page", "index");

    let hub_upload_path = Path::new(crate::UPLOAD_PATH).join(format!("{}/", &user.hub_id));

    if !hub_upload_path.exists() {
        if std::fs::create_dir_all(&hub_upload_path).is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }

    let mut entries = match std::fs::read_dir(&hub_upload_path) {
        Ok(read_dir) => read_dir
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().into_string())
            .filter_map(|e| e.ok())
            .collect(),
        Err(_) => vec![],
    };

    entries.sort();

    context.insert("entries", &entries);

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
) -> impl Responder {
    let alerts = flash_messages
        .iter()
        .map(|f| (f.content(), alert_level_to_str(&f.level())))
        .collect::<Vec<_>>();
    let mut context = Context::new();
    context.insert("alerts", &alerts);
    context.insert("current_user", &user);
    context.insert("current_page", "index");

    render_template("main/not_assigned.html", &context)
}

#[post("/upload_image")]
pub async fn upload_image(
    user: AuthenticatedUser,
    MultipartForm(form): MultipartForm<UploadFileForm>,
) -> impl Responder {
    let file_name = form
        .image
        .file_name
        .unwrap_or(format!("upload-{}", Uuid::new_v4()));

    let hub_upload_path = Path::new(crate::UPLOAD_PATH).join(format!("{}/", &user.hub_id));

    if !hub_upload_path.exists() {
        if std::fs::create_dir_all(&hub_upload_path).is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }

    let filepath = hub_upload_path.join(file_name);

    match form.image.file.persist(filepath) {
        Ok(_) => FlashMessage::success("Файл успешно загружен.").send(),
        Err(_) => FlashMessage::error("Ошибка при загрузке файла.").send(),
    }

    redirect("/")
}
