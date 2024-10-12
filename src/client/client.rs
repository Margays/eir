use serde_json::Value;
use std::error::Error;

pub trait Client {
    fn get(&self, url: &str) -> Result<Value, dyn Error>;
}
