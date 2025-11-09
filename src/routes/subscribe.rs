use actix_web::{HttpResponse, web};
use log::log;
use sqlx::types::uuid;
use sqlx_postgres::{PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    log!(
        log::Level::Info,
        "New subscriber: {} {}",
        _form.email,
        _form.name
    );
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3,$4)"#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        chrono::Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            log!(log::Level::Error, "Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
