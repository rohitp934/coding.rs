use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;
// use rustycoding::code_checker;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let logger = Logger::default();
        App::new().wrap(logger).service(index).service(check)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
