extern crate log;

use crate::config::config::read_config;
use crate::server::handler::Handler;
use crate::server::server::Server;

mod config;
mod server;

pub async fn start(config_file_path: &str) {
    let config = read_config(config_file_path);
    let server = Server::new(config.server, Handler {});

    server.start().await
}
