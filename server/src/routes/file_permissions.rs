use crate::error::{ResponseResult, ToCommandError, ToFileIOError};
use crate::models::File;
use actix_web::{get, post, web, HttpResponse};
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

#[post("/set-perms")]
async fn set_perms(file_payload: web::Json<File>) -> ResponseResult {
    let path = &file_payload.path;

    let file = tokio::fs::File::open(path)
        .await
        .map_fileio(path.to_str().unwrap_or_default())?;

    file.set_permissions(Permissions::from_mode(file_payload.mode.unwrap_or(0o400)))
        .await
        .map_command("chmod")?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/get-perms")]
async fn get_perms(file_payload: web::Json<File>) -> ResponseResult {
    let path = &file_payload.path;
    let perms = std::fs::metadata(path)
        .map_fileio(path.to_str().unwrap_or_default())?
        .permissions()
        .mode();
    let perms_octal = format!("{:o}", perms).parse::<u32>().unwrap();
    Ok(HttpResponse::Ok().json(perms_octal))
}
