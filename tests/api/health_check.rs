use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_sanity() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Request failed");
    assert!(resp.status().is_success());
    assert_eq!(resp.content_length(), Some(0));
}
