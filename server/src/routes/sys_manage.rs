use std::{fs::Permissions, os::unix::prelude::PermissionsExt};

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
    let cmd_str = if ssh_payload.toggle {
        "enable --now"
    } else {
        "stop"
    };

    let mut cmd = Command::new("systemctl");
    cmd.args(&[cmd_str, "ssh"]);

    if let Err(e) = cmd.output().await {
        warn!("Could not run command systemctl {} ssh: {}", cmd_str, e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    if !ssh_payload.toggle {
        let mut disable_cmd = Command::new("systemctl");
        cmd.args(&["disable", "ssh"]);

        if let Err(e) = disable_cmd.output().await {
            warn!("Could not run command systemctl disable ssh: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }

    HttpResponse::Ok().finish()
}

#[post("/usb")]
async fn usb(usb_payload: web::Json<Toggle>) -> impl Responder {
    let media_dir = match tokio::fs::File::open("/media/").await {
        Ok(d) => d,
        Err(e) => {
            warn!("Could not open /media/: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };
    let mode = if usb_payload.toggle {
        0o777
    } else {
        0o000
        // sudo echo "blacklist usb-storage" >> /etc/modprobe.d/blacklist.conf
    };

    if let Err(e) = media_dir
        .set_permissions(Permissions::from_mode(mode))
        .await
    {
        warn!("Error setting permissions of /media/: {}", e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().finish()
}
