use super::util;
use crate::error::{ResponseResult, ToCommandError, ToServiceError};
use crate::models::Toggle;
use actix_web::{get, post, web, HttpResponse};

#[post("/toggle")]
async fn toggle(ssh_payload: web::Json<Toggle>) -> ResponseResult {
    util::toggle_service("sshd", ssh_payload.toggle)
        .await
        .map_service("sshd")?;

    Ok(HttpResponse::Ok().into())
}

#[get("/status")]
async fn status() -> ResponseResult {
    let is_active = util::is_active("sshd")
        .await
        .map_command("systemctl is-active")?;

    Ok(HttpResponse::Ok().body(is_active.to_string()))
}
