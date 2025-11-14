use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;
use log::{Metadata, Record};
use sqlx_postgres::{PgPool};
use crate::routes::{heath_check, subscribe};

pub mod routes;
pub mod configuration;
pub mod startup;

pub mod telemetry;

pub trait Log: Sync + Send {

    // 是否应该被启用
    fn enable(&self,metadata: &Metadata) -> bool;

    // 记录日志
    fn log(&self, record: &Record);

    // 提交所有缓存的日志
    fn flush(&self);
}

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
