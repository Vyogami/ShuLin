use crate::models::File;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::warn;
use std::fmt::format;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

#[post("/set-perms")]
async fn set_perms(file_payload: web::Json<File>) -> impl Responder {
    let file = match tokio::fs::File::open(&file_payload.path).await {
        Ok(f) => f,
        Err(e) => {
            warn!(
                "Error while opening file {}: {}",
                file_payload.path.display(),
                e.to_string()
            );
            return HttpResponse::NotFound().finish();
        }
    };

    if let Err(e) = file
        .set_permissions(Permissions::from_mode(file_payload.mode.unwrap_or(0o400)))
        .await
    {
        warn!("Error while setting permissions: {}", e);
        return HttpResponse::BadRequest().json(e.to_string());
    }

    HttpResponse::Ok().finish()
}

#[get("/get-perms")]
async fn get_perms(file_payload: web::Json<File>) -> Result<HttpResponse, actix_web::error::Error> {
    match std::fs::metadata(&file_payload.path) {
        Ok(metadata) => {
            let perms = metadata.permissions().mode();
            let perms_octal = format!("{:o}", perms).parse::<u32>().unwrap();
            Ok(HttpResponse::Ok().json(perms_octal))
        }
        Err(err) => {
            warn!(
                "Error while getting permissions for {}: {}",
                file_payload.path.to_str().unwrap_or_default(),
                err
            );
            Ok(HttpResponse::NotFound().json(err.to_string()))
        }
    }
}
