mod routes;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use routes::config;

// use rustycoding::execute;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let logger = Logger::new("\"%r\" %s (%b bytes) %Tms");
        App::new().wrap(logger).configure(config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
