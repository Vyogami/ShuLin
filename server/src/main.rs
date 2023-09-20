pub mod models;
pub mod routes;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%t \"%r\" %s {{%P}} %T"))
            .service(ping)
            .service(routes::file_permissions::get_perms)
            .service(routes::file_permissions::set_perms)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
