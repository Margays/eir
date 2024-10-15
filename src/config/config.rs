use serde::Deserialize;

use super::client::Client;
use super::endpoint::Endpoint;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client: Client,
    pub endpoints: Vec<Endpoint>,
}

pub fn load_config() -> Config {
    let content = std::fs::read_to_string("config.yaml").unwrap();
    let config: Config = serde_yml::from_str(&content).unwrap();
    config
}
