use std::net::TcpListener;
use zero2prod::percent_encode_kvps;

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

#[tokio::test]
async fn subscribe_with_valid_form_data_returns_200() {
    // Arrange
    let address: String = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let name: &str = "Le Guin";
    let email: &str = "ursula_le_guin@gmail.com";
    let params: [(&str, &str); 2] = [("name", &name), ("email", &email)];
    let body: String = zero2prod::percent_encode_kvps(params);
    assert_eq!(
        &body,
        "name=Le%20Guin&email=ursula%5Fle%5Fguin%40gmail%2Ecom"
    );

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases: Vec<(String, &str)> = vec![
        (percent_encode_kvps([("name", "le guin")]), "missing the email"),
        (percent_encode_kvps([("email", "ursula_le_guin@gmail.com")]), "missing the name"),
        ("".to_owned(), "missing both the name and the email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // additional customized error message on test failure
            "the api did not fail with 400 Bad Request when the payload was {}",
            error_message,
        );
    }
}