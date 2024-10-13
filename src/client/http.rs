use serde_json::Value;
use super::client::Client;
use std::{collections::HashMap, error::Error};
use reqwest::header::HeaderMap;

pub struct HttpClient {
    api_client: reqwest::blocking::Client,
    extra_headers: HeaderMap
}

impl HttpClient {
    pub fn new(headers: &HashMap<String, String>) -> HttpClient {
        let api_client = reqwest::blocking::Client::new();
        let mut extra_headers = HeaderMap::new();
        for (key, value) in headers {
            extra_headers.append(
                reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
                reqwest::header::HeaderValue::from_str(&value).unwrap()
            );
        }
        HttpClient {
            api_client: api_client,
            extra_headers: extra_headers
        }
    }
}

impl Client for HttpClient {
    fn get(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let request = self.api_client.get(url)
            .headers(self.extra_headers.clone());
        let response = request.send();
        let json = response?.json()?;
        Ok(json)
    }
}
