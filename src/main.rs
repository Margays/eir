use jsonpath_rust::JsonPath;
use ::metrics::gauge;
use metrics::init_metrics;
use serde_json::Value;
use std::str::FromStr;

mod metrics;
mod config;

fn load_response() -> Value {
    let response = std::fs::read_to_string("response.json").unwrap();
    serde_json::from_str(&response).unwrap()
}

fn main() {
    let config = config::config::load_config();
    init_metrics(&3000, &config);

    for endpoint in &config.endpoints {
        for metric in &endpoint.metrics {
            let response = load_response();
            let path = JsonPath::from_str(metric.json_path.as_str()).unwrap();
            let val = path.find_slice(&response);
            let gauge = gauge!(metric.name.clone());
            gauge.set(val[0].clone().to_data().as_f64().unwrap());
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(60));
}
