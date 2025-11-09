use email_newsletter_service::run;
use email_newsletter_service::configuration::get_configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("{}:{}", configuration.application_host, configuration.application_port);
    let listener: TcpListener =
        TcpListener::bind(address).expect("Failed to bind port");
    run(listener)?.await
}
