use super::Client;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::{collections::HashMap, error::Error};
use tokio::sync::Semaphore;
use std::sync::Arc;

#[derive(Clone)]
pub struct HttpClient {
    api_client: reqwest::Client,
    extra_headers: HeaderMap,
    semaphore: Arc<Semaphore>,
}

impl HttpClient {
    pub fn new(headers: &HashMap<String, String>, max_conn: u32) -> HttpClient {
        let api_client = reqwest::Client::new();
        let mut extra_headers = HeaderMap::new();
        for (key, value) in headers {
            extra_headers.append(
                reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
                reqwest::header::HeaderValue::from_str(value).unwrap(),
            );
        }
        HttpClient {
            api_client,
            extra_headers,
            semaphore: Arc::new(Semaphore::new(max_conn as usize)),
        }
    }
}

impl Client for HttpClient {
    async fn get(&self, url: &str) -> Result<Value, Box<dyn Error>> {
        let request = self.api_client.get(url).headers(self.extra_headers.clone());
        let permit = self.semaphore.acquire().await;
        let response = request.send().await?;
        let json = response.json().await?;
        drop(permit);
        Ok(json)
    }
}
