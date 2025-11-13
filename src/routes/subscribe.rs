use actix_web::{HttpResponse, web};
use log::log;
use sqlx::types::uuid;
use sqlx_postgres::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - New subscriber: {} {}",
        request_id,
        _form.email,
        _form.name
    );
    tracing::info!(
        "request_id {} - Saving new subscriber detail in database",
        request_id
    );
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3,$4)"#,
        request_id,
        _form.email,
        _form.name,
        chrono::Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {} - New subscriber has been saved!", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
