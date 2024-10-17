use crate::client::Client;
use crate::config::endpoint::Endpoint;
use crate::config::metric::Label;
use ::metrics::gauge;
use jsonpath_rust::JsonPath;
use metrics::init_metrics;
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

mod client;
mod config;
mod metrics;

fn resolve_labels(labels: &Vec<Label>, response: &Value) -> Vec<(String, String)> {
    let mut resolved_labels = Vec::new();
    for label in labels {
        let name = label.name.clone();
        let value = label.value.clone();
        if value.starts_with("$") {
            let path = JsonPath::from_str(value.as_str()).unwrap();
            let val = path.find_slice(response);
            resolved_labels.push((name, val[0].clone().to_data().to_string()));
        } else {
            resolved_labels.push((name, value));
        }
    }
    resolved_labels
}

fn parse_interval(interval: &str) -> u64 {
    let mut value = interval.to_string();
    let unit = value.pop().unwrap();
    let value = value.parse::<u64>().unwrap();
    match unit {
        's' => value,
        'm' => value * 60,
        'h' => value * 60 * 60,
        _ => panic!("Invalid interval unit"),
    }
}

async fn fetch_metrics(client: Arc<client::http::HttpClient>, endpoint: Arc<Endpoint>) {
    loop {
        let response = client.get(endpoint.url.as_str()).await.unwrap();
        for metric in &endpoint.metrics {
            let path = JsonPath::from_str(metric.json_path.as_str()).unwrap();
            let val = path.find_slice(&response);
            let value = val[0].clone().to_data().as_f64().unwrap();
            let labels = resolve_labels(&metric.labels.clone(), &response);
            let gauge = gauge!(metric.name.clone(), &labels);
            gauge.set(value);
        }
        let interval = parse_interval(endpoint.interval.as_str());
        sleep(Duration::from_secs(interval)).await;
    }
}

#[tokio::main]
async fn main() {
    let config = config::load_config();
    init_metrics(&3000, &config);

    let _client = client::http::HttpClient::new(
        &config.client.headers,
        config.client.max_connections,
    );
    let mut tasks = Vec::new();
    for endpoint in &config.endpoints {
        let endpoint = Arc::new(endpoint.clone());
        let client = Arc::new(_client.clone());
        let task = tokio::spawn(async move {
            fetch_metrics(client, endpoint).await;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}
