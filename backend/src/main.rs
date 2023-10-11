use crate::config::config::read_config;
use crate::server::handler::Handler;
use crate::server::server::Server;

mod config;
mod server;

#[tokio::main]
async fn main() {
    let config = read_config("backend/config.toml");
    let server = Server::new(config.server, Handler {});

    server.start().await
}
