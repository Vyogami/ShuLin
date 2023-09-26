use super::util;
use crate::models::Toggle;
use actix_web::{post, web, HttpResponse, Responder};
use log::warn;

#[post("/toggle")]
async fn toggle(tor_payload: web::Json<Toggle>) -> impl Responder {
    if let Err(e) = util::toggle_service("tor", tor_payload.toggle).await {
        warn!("Could not run command systemctl for tor: {}", e);
        HttpResponse::InternalServerError().body(e.to_string())
    } else {
        HttpResponse::Ok().finish()
    }
}
