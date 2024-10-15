use serde::Deserialize;

use super::metric::Metric;

#[derive(Deserialize, Debug)]
pub struct Endpoint {
    pub url: String,
    pub metrics: Vec<Metric>,
}
