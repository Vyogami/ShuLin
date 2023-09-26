use actix_web::{post, HttpResponse, Responder};
use log::warn;
use tokio::process::Command;

#[post("/shutdown")]
async fn shutdown() -> impl Responder {
    let mut cmd = Command::new("shutdown");

    cmd.args(["-h", "now"]);

    if let Err(e) = cmd.output().await {
        warn!("Could not run command 'shutdown -h now' : {}", e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}
#[post("/reboot")]
async fn reboot() -> impl Responder {
    let mut cmd = Command::new("shutdown");

    cmd.args(["-r", "now"]);

    if let Err(e) = cmd.output().await {
        warn!("Could not run command 'shutdown -r now' : {}", e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}
