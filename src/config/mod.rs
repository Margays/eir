pub mod client;
pub mod endpoint;
pub mod metric;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub client: client::Client,
    pub endpoints: Vec<endpoint::Endpoint>,
}

pub fn load_config(path: &str) -> Config {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let config = serde_json::from_reader(reader).unwrap();
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config("config.json");
        assert!(!config.endpoints.is_empty());
        assert!(!config.client.headers.is_empty());
    }
}
