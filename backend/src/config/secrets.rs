use std::fs::read_to_string;

use serde::Deserialize;
use toml::from_str;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Secrets {
    pub client: Client
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct Client {
    pub key: String
}

pub fn read_secrets(file_path: &str) -> Secrets {
    let contents = read_to_string(file_path).unwrap();
    from_str(&contents).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let config = super::read_secrets("test_secrets.toml");
        assert_eq!(
            config,
            super::Secrets {
                client: super::Client {
                    key: "api_key_here".to_string()
                }
            }
        );
    }

    #[test]
    #[should_panic]
    fn it_panics_on_invalid_file() {
        super::read_secrets("nonsense.toml");
    }
}
