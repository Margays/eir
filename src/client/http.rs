use serde_json::Value;
use super::client::Client;
use serde_json::Error;

pub struct HttpClient {
    client: reqwest::blocking::Client,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Client for HttpClient {
    fn get(&self, url: &str) -> Result<Value, Error> {
        self.client.get(url).send()?
        
    }
}
