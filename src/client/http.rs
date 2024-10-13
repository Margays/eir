use serde_json::Value;
use super::client::Client;
//use serde_json::Error;
use std::error::Error;

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
    fn get(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let response = self.client.get(url).send();
        let json = response?.json()?;
        Ok(json)
    }
}
