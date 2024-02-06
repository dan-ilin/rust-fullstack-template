use common::Example;
use reqwest::StatusCode;

mod setup;

#[tokio::test]
async fn hello_world_route() {
    setup::setup();
    let response = reqwest::Client::new()
        .get(format!("http://localhost:8000/api/v1/hello/{}", "world"))
        .send()
        .await
        .unwrap()
        .json::<Example>()
        .await
        .unwrap();

    assert_eq!(
        response,
        Example {
            string: "world".to_string(),
            int: 12345,
            float: 67.890,
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
