use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Client {
    pub headers: HashMap<String, String>,
    pub max_connections: u32,
}
