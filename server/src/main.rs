pub mod models;
pub mod routes;

use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
            .service(
                web::scope("/sys")
                    .service(
                        web::scope("/ssh")
                            .service(routes::sys_manage::ssh::status)
                            .service(routes::sys_manage::ssh::toggle),
                    )
                    .service(web::scope("/usb").service(routes::sys_manage::usb::toggle))
                    .service(
                        web::scope("/tor")
                            .service(routes::sys_manage::tor::status)
                            .service(routes::sys_manage::tor::toggle),
                    ),
            )
            .service(
                web::scope("/file-permissions")
                    .service(web::scope("/get").service(routes::file_permissions::get_perms))
                    .service(web::scope("/set").service(routes::file_permissions::set_perms)),
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
