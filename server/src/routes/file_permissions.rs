use std::fs::metadata;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

use crate::models::File;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/set-perms")]
async fn set_perms(file_payload: web::Json<File>) -> impl Responder {
    let file = match tokio::fs::File::open(&file_payload.path).await {
        Ok(f) => f,
        Err(_) => return HttpResponse::NotFound(),
    };

    if let Some(mode) = file_payload.mode {
        if let Err(_) = file.set_permissions(Permissions::from_mode(mode)).await {
            return HttpResponse::BadRequest();
        }
    } else {
        if let Err(_) = file.set_permissions(Permissions::from_mode(0o400)).await {
            return HttpResponse::BadRequest();
        }
    }

    HttpResponse::Ok()
}

#[get("/get_perms")]
async fn get_perms(file_payload: web::Json<File>) -> Result<HttpResponse, actix_web::error::Error> {
    match metadata(&file_payload.path) {
        Ok(metadata) => {
            let perms = metadata.permissions().mode();
            Ok(HttpResponse::Ok().json(perms))
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
