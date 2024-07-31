// json.rs

use serde_json::Value;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomObject {
    pub id: u32,
    pub name: String,
    pub details: Details,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub description: String,
    pub nested_info: NestedInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NestedInfo {
    pub info: String,
}

pub fn load_json_file<P: AsRef<Path>>(path: P) -> Result<Value, serde_json::Error> {
    let file = File::open(path).expect("file should open read only");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader)?;
    Ok(json_data)
}

pub fn create_custom_object(json_data: &Value) -> CustomObject {
    serde_json::from_value(json_data.clone()).expect("JSON was not well-formatted")
}

pub fn get_nested_value<'a>(json_data: &'a Value, keys: &[&str]) -> Option<&'a Value> {
    let mut current = json_data;
    for key in keys {
        current = current.get(*key)?;
    }
    Some(current)
}

