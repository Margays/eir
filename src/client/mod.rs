pub mod http;

use serde_json::Value;
use std::error::Error;

pub trait Client {
    async fn get(&self, url: &str) -> Result<Value, Box<dyn Error>>;
}
