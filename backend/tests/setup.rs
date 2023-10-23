lazy_static::lazy_static! {
    static ref SETUP: bool = {
        println!("Starting integration tests");
        setup();
        true
    };
}

async fn start_backend() {
    backend::start().await
}


pub fn setup() {
    tokio::spawn(start_backend());
}
