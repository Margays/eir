use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

use crate::config::Config;
use crate::config::metric::MetricType;

pub fn init_metrics(config: &Config, port: u16) {
    println!("initializing metrics exporter");

    PrometheusBuilder::new()
        .idle_timeout(
            MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
            Some(Duration::from_secs(10)),
        )
        .with_http_listener(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))
        .install()
        .expect("failed to install metrics exporter");

    for endpoint in &config.endpoints {
        for metric in &endpoint.metrics {
            match metric.r#type {
                MetricType::Counter => {
                    metrics::describe_counter!(metric.name.clone(), metric.description.clone());
                }
                MetricType::Gauge => {
                    metrics::describe_gauge!(metric.name.clone(), metric.description.clone());
                }
                MetricType::Histogram => {
                    metrics::describe_histogram!(metric.name.clone(), metric.description.clone());
                }
            }
        }
    }
}
