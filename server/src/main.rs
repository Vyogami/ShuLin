pub mod models;
use std::os::unix::fs::PermissionsExt;
use std::fs::Permissions;
use std::fs::metadata;


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::File;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/set-perms")]
async fn set_perms(file_payload: web::Json<File>) -> impl Responder {
    let file = match tokio::fs::File::open(&file_payload.path).await {
        Ok(f) => f,
        Err(_) => return HttpResponse::NotFound(),
    };

    
    if let Err(_) = file
        .set_permissions(Permissions::from_mode(file_payload.mode))
        .await
    {
        return HttpResponse::BadRequest();
    }

    HttpResponse::Ok()
}

#[get("/get_perms")]
async fn get_perms(file_payload: web::Json<File>)->Result<HttpResponse,actix_web::error::Error>{
    
    
    
    match metadata(&file_payload.path) {
        Ok(metadata) => {
            let perms = metadata.permissions().mode();
            Ok(HttpResponse::Ok().json(perms))
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            Ok(HttpResponse::NotFound().finish())
        }
    }
    
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping))
        .bind(("127.0.0.1", 5432))?
        .run()
        .await
}
