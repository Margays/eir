use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

use crate::config::config::Config;
use crate::config::metric::{Metric, MetricType};


pub fn init_metrics(port: &u16, config: &Config) {
    println!("initializing metrics exporter");

    PrometheusBuilder::new()
        .idle_timeout(
            MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
            Some(Duration::from_secs(10)),
        )
        .with_http_listener(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port.to_owned(),
        ))
        .install()
        .expect("failed to install Prometheus recorder");

    for endpoint in &config.endpoints {
        for metric in &endpoint.metrics {
            match metric.r#type {
                MetricType::Counter => register_counter(metric),
                MetricType::Gauge => register_gauge(metric),
                MetricType::Histogram => register_histogram(metric),
            }
        }
    }
}

/******** Utils ********/

/// Registers a counter with the given name.
fn register_counter(metric: &Metric) {
    metrics::describe_counter!(metric.name.clone(), metric.description.clone());
    let _counter = metrics::counter!(metric.name.clone());
}

/// Registers a gauge with the given name.
fn register_gauge(metric: &Metric) {
    metrics::describe_gauge!(metric.name.clone(), metric.description.clone());
    let _gauge = ::metrics::gauge!(metric.name.clone());
}

/// Registers a histogram with the given name.
fn register_histogram(metric: &Metric) {
    metrics::describe_histogram!(metric.name.clone(), metric.description.clone());
    let _histogram = ::metrics::histogram!(metric.name.clone());
}
