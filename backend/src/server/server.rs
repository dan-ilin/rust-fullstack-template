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
        println!("Starting server at {}", address);
        warp::serve(self.handler.build_routes()).run(address).await
    }
}
