use email_newsletter_service::run;
use email_newsletter_service::configuration::get_configuration;
use std::net::TcpListener;
use sqlx_postgres::{PgPool};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("{}:{}", configuration.application_host, configuration.application_port);
    let listener: TcpListener =
        TcpListener::bind(&address).expect("Failed to bind port");
    run(listener, connection_pool)?.await
}
