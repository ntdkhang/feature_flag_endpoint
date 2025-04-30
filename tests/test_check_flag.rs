use std::net::TcpListener;
use feature_flag_endpoint::{run, FeatureFlagData};
use reqwest::Client;

#[tokio::test]
async fn test_check_flag_enabled() {
    let address = spawn_app();
    let client = Client::new();

    let flag = client
        .get(format!("{}/check_flag", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .query(&[("user", "khang")])
        .send()
        .await
        .expect("Failed to get response")
        .json::<FeatureFlagData>()
        .await
        .expect("Failed to parse as FeatureFlagData");

    assert_eq!(flag.user, "khang");
    assert!(flag.enabled);
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind listener");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to create server");
    std::mem::drop(tokio::spawn(server));
    format!("http://127.0.0.1:{}", port)
}
