use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(check);
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Actix Web Rust Server is running!")
}
#[derive(Deserialize)]
struct CheckQuery {
    value: bool,
}

#[get("/check")]
async fn check(info: web::Query<CheckQuery>) -> impl Responder {
    if info.value {
        HttpResponse::Ok().body("Everything looks good!")
    } else {
        HttpResponse::BadRequest().body("You messed up fr!")
    }
}
