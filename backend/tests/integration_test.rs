use common::Example;

mod setup;

#[tokio::test]
async fn it_adds_two() {
    setup::setup();
    let response = reqwest::Client::new()
        .get(format!("http://127.0.0.1:80/api/v1/hello/{}", "world"))
        .send()
        .await
        .unwrap()
        .json::<Example>()
        .await
        .unwrap();
    assert_eq!(response, Example {
        string: "world".to_string(),
        int: 12345,
        float: 67.890,
    });
}
