use axum::{routing::get, Json, Router};
use common::Example;

pub struct Handler {}

impl Handler {
    pub fn build_router(self) -> Router {
        Router::new().route("/api/v1", get(root))
    }
}

async fn root() -> Json<Example> {
    Json(Example {
        string: "hello world".to_string(),
        int: 1234567890,
        float: 12345.67890,
    })
}
