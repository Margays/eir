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

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub clients: HashMap<String, client::Client>,
    pub endpoint_groups: HashMap<String, Vec<endpoint::Endpoint>>,
    pub contexts: HashMap<String, Context>,
}

impl Config {
    pub fn get_endpoint_group(&self, name: &str) -> Option<&Vec<endpoint::Endpoint>> {
        self.endpoint_groups.get(name)
    }

    pub fn validate(&self) -> bool {
        !self.clients.is_empty() && !self.endpoint_groups.is_empty() && !self.contexts.is_empty()
    }

    fn load<T>(dir: PathBuf) -> HashMap<String, T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut entries: HashMap<String, T> = HashMap::new();
        if !dir.exists() || !dir.is_dir() {
            eprintln!(
                "Directory does not exist or is not a directory: {}",
                dir.display()
            );
            return entries;
        }

        // Load clients from the directory
        for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let file = std::fs::File::open(entry.path())
                    .unwrap_or_else(|_| panic!("Failed to open file: {}", entry.path().display()));
                let data: T = serde_json::from_reader(file)
                    .unwrap_or_else(|_| panic!("Failed to parse file: {}", entry.path().display()));
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

    #[test]
    fn test_empty_config() {
        let config = Config::default();
        assert!(config.clients.is_empty());
        assert!(config.endpoint_groups.is_empty());
        assert!(config.contexts.is_empty());
    }

    #[test]
    fn test_loading_from_nonexistent_directory() {
        let dir = PathBuf::from("nonexistent");
        let config: Config = Config::from(&dir);
        assert!(config.clients.is_empty());
        assert!(config.endpoint_groups.is_empty());
        assert!(config.contexts.is_empty());
    }

    #[test]
    fn test_loading_from_empty_directory() {
        let dir = PathBuf::from("empty");
        std::fs::create_dir_all(&dir).expect("Failed to create empty directory");
        let config: Config = Config::from(&dir);
        assert!(config.clients.is_empty());
        assert!(config.endpoint_groups.is_empty());
        assert!(config.contexts.is_empty());
        std::fs::remove_dir_all(&dir).expect("Failed to remove empty directory");
    }
}
