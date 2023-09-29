use crate::models::Toggle;
use actix_web::{post, web, HttpResponse, Responder};
use tokio::process::Command;
//
// #[post("/toggle")]
// async fn toggle(usb_payload: web::Json<Toggle>) -> impl Responder {
//     if usb_payload.toggle {
//         let mut cmd = Command::new("modprobe");
//         cmd.arg("usb-storage");
//
//         if let Err(e) = cmd.output().await {
//             warn!("Could not run command modprobe usb-storage: {}", e);
//             return HttpResponse::InternalServerError().body(e.to_string());
//         }
//     } else {
//         let mut modprobe_blacklist = match fs::OpenOptions::new()
//             .append(true)
//             .open("/etc/modprobe.d/blacklist.conf")
//             .await
//         {
//             Ok(f) => f,
//             Err(e) => {
//                 warn!("Could not open modproble blacklist file: {}", e);
//                 return HttpResponse::InternalServerError().body(e.to_string());
//             }
//         };
//
//         use tokio::io::AsyncWriteExt;
//         if let Err(e) = modprobe_blacklist
//             .write("blacklist usb-storage".as_bytes())
//             .await
//         {
//             warn!("Could not write config to modproble blacklist file: {}", e);
//             return HttpResponse::InternalServerError().body(e.to_string());
//         }
//     }
//
//     HttpResponse::Ok().finish()
// }
//

#[post("/toggle")]
async fn toggle(usb_payload: web::Json<Toggle>) -> impl Responder {
    let drivers_path = "/lib/modules/6.5.5-arch1-1/kernel/drivers/usb/";
    let temp = format!("{drivers_path}temp_storage");
    let real = format!("{drivers_path}storage");

    if usb_payload.toggle {
        tokio::fs::rename(temp, real).await.unwrap();
    } else {
        tokio::fs::rename(real, temp).await.unwrap();
        let mut uas = Command::new("rmmod");
        uas.args(["--force", "uas"]);

        let mut usb = Command::new("rmmod");
        usb.args(["--force", "usb_storage"]);

        uas.output().await.unwrap();
        usb.output().await.unwrap();
    };

    HttpResponse::Ok().finish()
}
