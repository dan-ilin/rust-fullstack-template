use crate::config::config;
use crate::server::handler::Handler;

use warp::{serve, Filter, log};

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
        let log = warp::log("api");
        let routes = self.handler.build_routes().with(log);

        serve(routes).bind(address).await
    }
}
