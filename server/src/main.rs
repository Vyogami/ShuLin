pub mod models;
pub mod routes;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(routes::file_permissions::get_perms)
            .service(routes::file_permissions::set_perms)
    })
    .bind(("127.0.0.1", 5432))?
    .run()
    .await
}
