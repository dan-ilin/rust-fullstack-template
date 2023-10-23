#[tokio::main]
async fn main() {
    backend::start("backend/config.toml").await
}
