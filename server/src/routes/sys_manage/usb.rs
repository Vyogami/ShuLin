use crate::models::Toggle;
use actix_web::{post, web, HttpResponse, Responder};
use tokio::process::Command;

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
