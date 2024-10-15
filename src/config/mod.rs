pub mod client;
pub mod endpoint;
pub mod metric;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client: client::Client,
    pub endpoints: Vec<endpoint::Endpoint>,
}

pub fn load_config() -> Config {
    let content = std::fs::read_to_string("config.yaml").unwrap();
    let config: Config = serde_yml::from_str(&content).unwrap();
    config
}
