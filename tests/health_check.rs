#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    use std::net::TcpListener;

    // Bind to a random free port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    // Launch the server in the background
    let server = zero2prod::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
