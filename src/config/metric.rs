use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Label {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub description: String,
    pub r#type: MetricType,
    pub jmes_expression: String,
    pub labels: Vec<Label>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
}
