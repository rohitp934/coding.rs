use actix_web::{get, Responder, HttpResponse, HttpServer, App};
// use rustycoding::code_checker;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Actix Web Rust Server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
