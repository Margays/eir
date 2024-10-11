use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

#[derive(Deserialize,Debug)]
pub struct Metric {
    pub name: String,
    pub description: String,
    pub r#type: MetricType,
    pub json_path: String,
    pub labels: Option<HashMap<String, String>>
}

#[derive(Debug)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram
}

impl<'de> Deserialize<'de> for MetricType {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let metric_type = String::deserialize(de)?;
        Ok(match metric_type.as_str() {
            "counter" => MetricType::Counter,
            "gauge" => MetricType::Gauge,
            "Histogram" => MetricType::Histogram,
            _ => panic!("Invalid metric type")
        })
    }
}
