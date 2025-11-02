use std::net::TcpListener;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn heath_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// #[actix_web::main]
pub fn run(tcp_listener: TcpListener) -> Result<Server,std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }))
            .route("/{name}", web::get().to(greet))
            .route("/health", web::get().to(heath_check))
    })
        .listen(tcp_listener)?
        .run();
    Ok(server)
}