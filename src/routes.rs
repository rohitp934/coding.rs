use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use rustycoding::{execute, types::Question};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(check);
    cfg.service(run);
}

#[get("/")]
async fn index() -> impl Responder {
    info!("Rusty coding at your service!");
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

#[post("/run")]
async fn run(body: web::Json<Question>) -> impl Responder {
    let question = body.into_inner();
    execute(question).await
}
