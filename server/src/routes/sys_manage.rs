use std::{fs::Permissions, os::unix::prelude::PermissionsExt};
use tokio::fs;

use crate::models::Toggle;
use actix_web::{post, web, HttpMessage, HttpResponse, Responder};
use log::warn;
use tokio::process::Command;

#[post("/sys-update")]
async fn sys_update() -> impl Responder {
    HttpResponse::InternalServerError()
}

#[post("/ssh")]
async fn ssh(ssh_payload: web::Json<Toggle>) -> impl Responder {
    let cmd_str = if ssh_payload.toggle {
        "enable"
    } else {
        "disable"
    };

    let mut cmd = Command::new("systemctl");
    cmd.args(&[cmd_str, "sshd"]);

    if let Err(e) = cmd.output().await {
        warn!("Could not run command systemctl {} ssh: {}", cmd_str, e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    if !ssh_payload.toggle {
        let mut disable_cmd = Command::new("systemctl");
        cmd.args(&["disable", "sshd"]);

        if let Err(e) = disable_cmd.output().await {
            warn!("Could not run command systemctl disable ssh: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        }
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
