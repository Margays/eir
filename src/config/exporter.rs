use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Exporter {
    pub port: u16,
}
