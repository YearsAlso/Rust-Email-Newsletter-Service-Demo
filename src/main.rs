use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use email_newsletter_service::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
