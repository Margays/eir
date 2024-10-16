use crate::client::Client;
use crate::config::metric::Label;
use ::metrics::gauge;
use jsonpath_rust::JsonPath;
use metrics::init_metrics;
use serde_json::Value;
use std::str::FromStr;
use std::thread;

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

fn main() {
    let config = config::load_config();
    init_metrics(&3000, &config);

    let api_client = client::http::HttpClient::new(&config.client.headers);
    thread::scope(|s| {
        for endpoint in &config.endpoints {
            s.spawn(|| {
                loop {
                    let response = api_client.get(endpoint.url.as_str()).unwrap();
                    for metric in &endpoint.metrics {
                        let path = JsonPath::from_str(metric.json_path.as_str()).unwrap();
                        let val = path.find_slice(&response);
                        let value = val[0].clone().to_data().as_f64().unwrap();
                        let labels = resolve_labels(&metric.labels.clone(), &response);
                        let gauge = gauge!(metric.name.clone(), &labels);
                        gauge.set(value);
                    }
                    let interval = parse_interval(endpoint.interval.as_str());
                    thread::sleep(std::time::Duration::from_secs(interval));
                    println!("Sleeping for {} seconds", interval);
                }
            });
        }
    });
}
