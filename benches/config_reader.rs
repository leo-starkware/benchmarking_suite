use std::fs;
use serde::{Serialize,Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct RawInputs {
    pub urls: Vec<String>,
    pub methods: Vec<String>,
}

impl RawInputs {
    pub fn new_from_json(path: &str) -> RawInputs {
        let contents = fs::read_to_string(path).expect("Config file not found");
        let deserialized: RawInputs = serde_json::from_str(contents.as_str()).unwrap();
        deserialized
    }
}