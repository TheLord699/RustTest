use std::fs;
use serde_json::Value;

pub fn load_json(path: &str) -> Value {
    let content = fs::read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&content).expect("JSON was not well-formatted")
}
