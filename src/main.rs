use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn heath_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }))
            .route("/{name}", web::get().to(greet))
            .route("/health", web::get().to(heath_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use crate::heath_check;

    #[tokio::test]
    async fn heath_check_succeeds(){
        let repose = heath_check().await ;
        assert!(repose.status().is_success())
    }
}
