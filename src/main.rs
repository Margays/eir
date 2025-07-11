use crate::client::Client;
use crate::config::endpoint::Endpoint;
use crate::config::metric::{Label, MetricType};
use ::metrics::gauge;
use metrics::init_metrics;
use serde_json::Value;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

mod client;
mod config;
mod jmes_extensions;
mod metrics;

fn extract_value(data: &Value, path: &str) -> String {
    if path.starts_with("{{") && path.ends_with("}}") {
        let jmes_path = crate::jmes_extensions::compile(&path[2..path.len() - 2]).unwrap();
        let jmes_value = jmespath::Variable::from_serializable(data).unwrap();
        let value = jmes_path.search(&jmes_value).unwrap();
        value.to_string()
    } else {
        path.to_string()
    }
}

fn resolve_labels(labels: &Vec<Label>, response: &Value) -> Vec<(String, String)> {
    let mut resolved_labels = Vec::new();
    for label in labels {
        let name = label.name.clone();
        let value = label.value.clone();
        resolved_labels.push((name, extract_value(response, &value)));
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
            let raw_value = extract_value(&response, &metric.jmes_expression);
            let value: f64 = raw_value.parse().unwrap();
            let labels = resolve_labels(&metric.labels.clone(), &response);
            match &metric.r#type {
                MetricType::Counter => {
                    let counter = gauge!(metric.name.clone(), &labels);
                    counter.set(value);
                }
                MetricType::Gauge => {
                    let gauge = gauge!(metric.name.clone(), &labels);
                    gauge.set(value);
                }
                MetricType::Histogram => {
                    let histogram = gauge!(metric.name.clone(), &labels);
                    histogram.set(value);
                }
            }
        }
        let interval = parse_interval(endpoint.interval.as_str());
        sleep(Duration::from_secs(interval)).await;
    }
}

#[tokio::main]
async fn main() {
    let config = config::load_config("config.json");
    init_metrics(&config);

    let http_client =
        client::http::HttpClient::new(&config.client.headers, config.client.max_connections);
    let mut tasks = Vec::new();
    for endpoint in &config.endpoints {
        let endpoint = Arc::new(endpoint.clone());
        let client = Arc::new(http_client.clone());
        let task = tokio::spawn(async move {
            fetch_metrics(client, endpoint).await;
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}
