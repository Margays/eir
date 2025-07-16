pub mod client;
pub mod endpoint;
pub mod metric;

use std::{collections::HashMap, path::PathBuf};

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

    fn load<T>(dir: PathBuf) -> HashMap<String, T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut entries: HashMap<String, T> = HashMap::new();
        // Load clients from the directory
        for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let file = std::fs::File::open(entry.path()).expect("Failed to open client file");
                let data: T = serde_json::from_reader(file).expect("Failed to parse client file");
                let filename = entry
                    .path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .expect("Failed to get filename")
                    .to_string();
                entries.insert(filename, data);
            }
        }
        entries
    }
}

impl From<&PathBuf> for Config {
    fn from(dir: &PathBuf) -> Self {
        let clients = Config::load::<client::Client>(dir.join("clients"));
        let endpoint_groups = Config::load::<Vec<endpoint::Endpoint>>(dir.join("endpoint_groups"));
        let contexts = Config::load::<Context>(dir.join("contexts"));

        Config {
            clients,
            endpoint_groups,
            contexts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let dir = PathBuf::from("example");
        let config: Config = Config::from(&dir);
        assert!(!config.get_endpoint_group("github").unwrap().is_empty());
        assert!(!config.clients["main"].headers.is_empty());
    }
}
