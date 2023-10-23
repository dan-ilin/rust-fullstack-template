lazy_static::lazy_static! {
    static ref SETUP: bool = {
        setup();
        true
    };
}

async fn start_backend() {
    backend::start("config.toml").await
}


pub fn setup() {
    tokio::spawn(start_backend());
}
