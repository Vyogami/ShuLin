use crate::error::{ResponseResult, ToCommandError};
use actix_web::{post, HttpResponse};
use tokio::process::Command;

#[post("/shutdown")]
async fn shutdown() -> ResponseResult {
    let mut cmd = Command::new("shutdown");

    cmd.args(["-h", "now"]);

    cmd.output().await.map_command("shutdown -h now")?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/reboot")]
async fn reboot() -> ResponseResult {
    let mut cmd = Command::new("shutdown");

    cmd.args(["-r", "now"]);

    cmd.output().await.map_command("shutdown -r now")?;

    Ok(HttpResponse::Ok().finish())
}
