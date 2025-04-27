// spawn_app is the only peice that depends on application code
// can ditch rust and replace with ruby for instance
// (test suite decoupled)
#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = backendrust::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
