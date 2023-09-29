use crate::{
    error::{ResponseResult, ToCommandError, ToModuleError},
    models::Toggle,
};
use actix_web::{post, web, HttpResponse};
use tokio::process::Command;

#[post("/toggle")]
async fn toggle(usb_payload: web::Json<Toggle>) -> ResponseResult {
    let mut cmd = Command::new("uname");
    cmd.arg("-r");

    let drivers_path = format!(
        "/lib/modules/{}/kernel/drivers/usb/",
        std::str::from_utf8(&cmd.output().await.map_command("uname -r")?.stdout)?
    );

    let temp = format!("{drivers_path}temp_storage");
    let real = format!("{drivers_path}storage");

    if usb_payload.toggle {
        tokio::fs::rename(temp, real)
            .await
            .map_module("usb_storage")?;
    } else {
        tokio::fs::rename(real, temp)
            .await
            .map_module("usb_storage")?;

        let mut uas = Command::new("rmmod");
        uas.args(["--force", "uas"]);

        let mut usb = Command::new("rmmod");
        usb.args(["--force", "usb_storage"]);

        uas.output().await.map_module("uas")?;
        usb.output().await.map_module("usb_storage")?;
    };

    Ok(HttpResponse::Ok().finish())
}
