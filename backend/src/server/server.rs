use crate::config::config;
use crate::server::handler::Handler;

pub struct Server {
    config: config::Server,
    handler: Handler,
}

impl Server {
    pub fn new(config: config::Server, handler: Handler) -> Self {
        Self { config, handler }
    }

    pub async fn start(self) {
        let address = self.config.get_full_address();
        let router = self.handler.build_router();
        tracing_subscriber::fmt::init();
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
}
