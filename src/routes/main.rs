use actix_identity::Identity;
use actix_web::{Responder, get, post};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::Context;

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
