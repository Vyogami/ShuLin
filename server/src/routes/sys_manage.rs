use tokio::fs;

use crate::models::Toggle;
use actix_web::{post, web, HttpResponse, Responder};
use log::warn;
use tokio::process::Command;

#[post("/sys-update")]
async fn sys_update() -> impl Responder {
    HttpResponse::InternalServerError()
}

#[post("/ssh")]
async fn ssh(ssh_payload: web::Json<Toggle>) -> impl Responder {
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

#[post("/usb")]
async fn usb(usb_payload: web::Json<Toggle>) -> impl Responder {
    if usb_payload.toggle {
        let mut cmd = Command::new("modprobe");
        cmd.arg("usb-storage");

        if let Err(e) = cmd.output().await {
            warn!("Could not run command modprobe usb-storage: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    } else {
        let mut modprobe_blacklist = match fs::OpenOptions::new()
            .append(true)
            .open("/etc/modprobe.d/blacklist.conf")
            .await
        {
            Ok(f) => f,
            Err(e) => {
                warn!("Could not open modproble blacklist file: {}", e);
                return HttpResponse::InternalServerError().body(e.to_string());
            }
        };

        use tokio::io::AsyncWriteExt;
        if let Err(e) = modprobe_blacklist
            .write("blacklist usb-storage".as_bytes())
            .await
        {
            warn!("Could not write config to modproble blacklist file: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }

    HttpResponse::Ok().finish()
}
