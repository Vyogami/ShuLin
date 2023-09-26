use crate::models::Toggle;
use actix_web::{post, web, HttpResponse, Responder};
use log::warn;
use tokio::process::Command;

#[post("/tor")]
async fn toggle(tor_payload: web::Json<Toggle>) -> impl Responder {
    let mut cmd = Command::new("systemctl");

    if tor_payload.toggle {
        cmd.args(["disable", "--now"]);
    } else {
        cmd.args(["enable", "--now"]);
    }

    cmd.arg("tor");

    if let Err(e) = cmd.output().await {
        warn!("Could not run command systemctl for Tor: {}", e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}
