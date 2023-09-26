use super::util;
use crate::models::Toggle;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::warn;

#[post("/toggle")]
async fn toggle(ssh_payload: web::Json<Toggle>) -> impl Responder {
    if let Err(e) = util::toggle_service("sshd", ssh_payload.toggle).await {
        warn!("Could not run command systemctl for ssh: {}", e);
        HttpResponse::InternalServerError().body(e.to_string())
    } else {
        HttpResponse::Ok().finish()
    }
}

#[get("/status")]
async fn status() -> impl Responder {
    match util::is_active("sshd").await {
        Ok(b) => HttpResponse::Ok().body(b.to_string()),
        Err(e) => {
            warn!("Error in systemctl is-active ssh: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
