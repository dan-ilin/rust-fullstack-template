use std::{fs::read_to_string, net::SocketAddr};

use serde::Deserialize;
use toml::from_str;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub server: Server,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Server {
    address: String,
    port: Option<u16>,
}

impl Server {
    pub fn get_full_address(&self) -> SocketAddr {
        let port = self.port.unwrap_or(8080);
        format!("{address}:{port}", address = self.address, port = port)
            .parse()
            .unwrap()
    }
}

pub fn read_config(file_path: &str) -> Config {
    let contents = read_to_string(file_path).unwrap();
    from_str(&contents).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let config = super::read_config("config.toml");
        assert_eq!(
            config,
            super::Config {
                server: super::Server {
                    address: "127.0.0.1".to_string(),
                    port: Some(8000),
                }
            }
        );
    }

    #[test]
    #[should_panic]
    fn it_panics_on_invalid_file() {
        super::read_config("nonsense.toml");
    }
}
