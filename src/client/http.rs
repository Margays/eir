use super::Client;
use reqwest::header::HeaderMap;
use serde_json::{Value, json};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use std::{collections::HashMap, error::Error};
use tokio::sync::Semaphore;

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
                reqwest::header::HeaderName::from_str(key).unwrap(),
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
        let permit = self.semaphore.acquire().await.unwrap();
        let response = request.send().await?;
        drop(permit);
        let status = response.status();
        if status != reqwest::StatusCode::OK {
            let text = response.text().await?;
            return Err(Box::new(RequestError {
                url: url.to_string(),
                status,
                message: format!("Request failed with status: {status}, response: {text}"),
            }));
        }
        let headers: Value = response
            .headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        let json_data: Value = response.json().await?;
        let result: Value = json!({
            "response": {
                "headers": headers,
                "json": json_data,
                "status": status.as_u16(),
            }
        });
        Ok(result)
    }
}

// Define request error for better error handling
#[derive(Debug)]
pub struct RequestError {
    pub url: String,
    pub status: reqwest::StatusCode,
    pub message: String,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request to {} failed with status {}: {}",
            self.url, self.status, self.message
        )
    }
}

impl std::error::Error for RequestError {}

#[cfg(test)]
mod tests {
    use mockito::Server;
    use tokio::task::JoinSet;

    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_http_client() {
        let requests_count = 100;
        let max_connections = 10;
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"test": "test"}"#)
            .create_async()
            .await;

        let url = server.url();

        let headers: HashMap<String, String> = HashMap::new();
        let client = Arc::new(HttpClient::new(&headers, max_connections));

        let mut set: JoinSet<Value> = JoinSet::new();
        for _ in 0..requests_count {
            let client = client.clone();
            let url = url.clone();
            set.spawn(async move { client.get(&format!("{url}/test")).await.unwrap() });
        }
        while let Some(out) = set.join_next().await {
            let response = out.unwrap();
            assert_eq!(response["response"]["json"]["test"], "test");
        }
        mock.expect(requests_count).assert();
    }
}
