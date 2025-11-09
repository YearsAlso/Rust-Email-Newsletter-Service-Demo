use actix_web::HttpResponse;

pub async fn heath_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}