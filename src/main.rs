use serde_json::{Value};
use std::fs::read_to_string;
use immigration::helper::*;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let json_file = read_to_string("common.json")?.parse::<String>()?;
    let json = serde_json::from_str::<Value>(&json_file)?;
    if json.is_object() {
        iterate_object(&json);
    }
    Ok(())
}
