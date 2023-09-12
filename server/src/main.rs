pub mod models;
use std::os::unix::fs::PermissionsExt;
use std::fs::Permissions;
use std::fs::metadata;


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::File;

mod routes{
    pub mod file_permissions;
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping))
        .bind(("127.0.0.1", 5432))?
        .run()
        .await
}
