use serde::Deserialize;

use super::metric::Metric;

#[derive(Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub url: String,
    pub interval: String,
    pub metrics: Vec<Metric>,
}
