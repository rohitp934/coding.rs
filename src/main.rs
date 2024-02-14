mod routes;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use routes::config;

// use rustycoding::execute;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let logger = Logger::new("\"%r\" %s (%b bytes) %Dms");
        App::new().wrap(logger).configure(config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
