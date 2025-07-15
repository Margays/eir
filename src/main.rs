use crate::client::Client;
use crate::config::endpoint::Endpoint;
use crate::config::metric::{Label, MetricType};
use ::metrics::gauge;
use clap::Parser;
use metrics::init_metrics;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

mod client;
mod config;
mod jmes_extensions;
mod metrics;

fn extract_value(data: &Value, value: &str) -> String {
    let prefix = "jmespath:";
    if let Some(expression) = value.strip_prefix(prefix) {
        let jmes_path = crate::jmes_extensions::compile(expression).unwrap();
        let jmes_value = jmespath::Variable::from_serializable(data).unwrap();
        let extracted_value = jmes_path.search(&jmes_value).unwrap();
        extracted_value.to_string()
    } else {
        value.to_string()
    }
}

fn resolve_labels(labels: &Vec<Label>, response: &Value) -> HashMap<String, String> {
    let mut resolved_labels = HashMap::new();
    for label in labels {
        let name = label.name.clone();
        let value = extract_value(response, &label.value);
        resolved_labels.insert(name, value);
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

fn add_env_args(response: &mut Value) {
    let env = std::env::vars_os();
    let mut env_vars = json!({});
    for (key, value) in env {
        if let Ok(key_str) = key.into_string() {
            if let Ok(value_str) = value.into_string() {
                env_vars[key_str] = json!(value_str);
            }
        }
    }
    response["env"] = env_vars;
}

async fn fetch_metrics(
    client_name: String,
    client: Arc<client::http::HttpClient>,
    endpoint: Arc<Endpoint>,
) {
    loop {
        let mut response = client.get(endpoint.url.as_str()).await.unwrap();
        add_env_args(&mut response);
        for metric in &endpoint.metrics {
            let raw_value = extract_value(&response, &metric.value);
            let value: f64 = raw_value.parse().unwrap();
            let mut labels = resolve_labels(&metric.labels.clone(), &response);
            labels.insert("client".to_string(), client_name.clone());
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

fn create_clients(config: &config::Config) -> HashMap<String, Arc<client::http::HttpClient>> {
    config
        .clients
        .iter()
        .map(|(name, client)| {
            let http_client = Arc::new(client::http::HttpClient::new(
                &client.headers,
                client.max_connections,
            ));
            (name.clone(), http_client)
        })
        .collect()
}

#[derive(Parser, Debug)]
#[clap(name = "Metrics Collector", version, author)]
struct CommandLineArgs {
    #[clap(
        short,
        long,
        default_value = "config.json",
        env = "EXPORTER_CONFIG_PATH"
    )]
    /// Path to the configuration file
    config_path: String,
    #[clap(short, long, default_value = "3000", env = "EXPORTER_PORT")]
    /// Port to run the HTTP server on (optional)
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = CommandLineArgs::parse();
    println!("Using configuration file: {}", args.config_path);
    let file = std::fs::File::open(&args.config_path).expect("Failed to open config file");
    let config: config::Config = config::Config::from(file);

    init_metrics(&config, args.port);

    let http_clients = create_clients(&config);
    let mut tasks = Vec::new();
    for (name, context) in &config.contexts {
        let client = http_clients.get(&context.client).expect("Client not found");
        for group_name in &context.endpoint_groups {
            let endpoints = config
                .get_endpoint_group(group_name)
                .expect("Endpoint group not found");
            for endpoint in endpoints {
                let client_name = name.clone();
                let endpoint = Arc::new(endpoint.clone());
                let client = Arc::clone(client);
                let task = tokio::spawn(async move {
                    fetch_metrics(client_name, client, endpoint).await;
                });
                tasks.push(task);
            }
        }
    }

    for task in tasks {
        task.await.unwrap();
    }
}
