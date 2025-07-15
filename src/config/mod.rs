pub mod client;
pub mod endpoint;
pub mod metric;

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Context {
    pub client: String,
    pub endpoint_groups: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub clients: HashMap<String, client::Client>,
    pub endpoint_groups: HashMap<String, Vec<endpoint::Endpoint>>,
    pub contexts: HashMap<String, Context>,
}

impl Config {
    pub fn get_endpoint_group(&self, name: &str) -> Option<&Vec<endpoint::Endpoint>> {
        self.endpoint_groups.get(name)
    }
}

impl From<std::fs::File> for Config {
    fn from(file: std::fs::File) -> Self {
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).expect("Failed to parse config file")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let file = std::fs::File::open("config.json").expect("Failed to open config file");
        let config: Config = Config::from(file);
        assert!(!config.get_endpoint_group("github").unwrap().is_empty());
        assert!(!config.clients["main"].headers.is_empty());
    }
}
