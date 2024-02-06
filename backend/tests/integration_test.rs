use common::Example;
use reqwest::StatusCode;

mod setup;

#[tokio::test]
async fn hello_world_route() {
    setup::setup();
    let response = reqwest::Client::new()
        .get("http://localhost:8000/api/v1")
        .send()
        .await
        .unwrap()
        .json::<Example>()
        .await
        .unwrap();

    assert_eq!(
        response,
        Example {
            string: "hello world".to_string(),
            int: 1234567890,
            float: 12345.67890,
        }
    );
}

#[tokio::test]
async fn not_found() {
    setup::setup();
    let response = reqwest::Client::new()
        .get("http://localhost:8000/api/v1/nonsense")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
