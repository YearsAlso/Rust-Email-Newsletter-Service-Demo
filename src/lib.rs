use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;
use sqlx_postgres::{PgPool};
use crate::routes::{heath_check, subscribe};

pub mod routes;
pub mod configuration;
pub mod startup;

pub fn run(tcp_listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }),
            )
            .route("/health_check", web::get().to(heath_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health", web::get().to(heath_check))
            .app_data(connection.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
