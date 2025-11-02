use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn heath_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }),
            )
            .route("/health_check", web::get().to(heath_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/greet/{name}", web::get().to(greet))
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
