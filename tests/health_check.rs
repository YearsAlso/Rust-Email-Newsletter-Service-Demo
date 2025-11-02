use email_newsletter_service::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let _address = spawn_app().await;
    let client = reqwest::Client::new();

    let url = &format!("{}/health_check", &_address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to run the server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
