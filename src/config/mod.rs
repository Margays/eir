pub mod client;
pub mod endpoint;
pub mod exporter;
pub mod metric;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client: client::Client,
    pub endpoints: Vec<endpoint::Endpoint>,
    pub exporter: exporter::Exporter,
}

pub fn load_config(path: &str) -> Config {
    let content = std::fs::read_to_string(path).unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();
    config
}
