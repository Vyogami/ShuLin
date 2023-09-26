use crate::models::Toggle;
use actix_web::{post, web, HttpResponse, Responder};
use log::warn;
use tokio::process::Command;

#[post("/toggle")]
async fn toggle(ssh_payload: web::Json<Toggle>) -> impl Responder {
    let mut cmd = Command::new("systemctl");

    if ssh_payload.toggle {
        cmd.args(["enable", "--now"]);
    } else {
        cmd.args(["disable", "--now"]);
    }

    cmd.arg("sshd");

    if let Err(e) = cmd.output().await {
        warn!("Could not run command systemctl for ssh: {}", e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}
