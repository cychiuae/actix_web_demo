use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger;

use actix_web_demo::controller::hello::{index, ping};

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(ping))
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .unwrap();
}
