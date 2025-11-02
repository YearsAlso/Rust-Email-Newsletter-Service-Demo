use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;
use crate::routes::{heath_check, subscribe};

pub mod routes;
pub mod configuration;
pub mod startup;

// #[actix_web::main]
pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }),
            )
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health", web::get().to(heath_check))
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
