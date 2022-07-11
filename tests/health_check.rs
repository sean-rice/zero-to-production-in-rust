use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address: String = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
fn spawn_app() -> String {
    let tcp_listener =
        TcpListener::bind("127.0.0.1:0").expect("failed to bind to localhost with random port.");
    let listener_port: u16 = tcp_listener.local_addr().unwrap().port();

    let server = zero2prod::run(vec![tcp_listener])
        .expect("failed to start server (likely address binding failure).");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{listener_port}")
}
