use actix_web::{post, HttpResponse, Responder};

#[post("/sys-update")]
async fn sys_update() -> impl Responder {
    HttpResponse::InternalServerError()
}