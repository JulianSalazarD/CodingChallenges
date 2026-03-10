use std::error::Error;
use std::fs;

pub mod json;

use crate::json::json::parse;

pub fn build(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let file_path = args.next().ok_or("No query provided")?;

    if file_path.ends_with(".json") {
        Ok(file_path)
    } else {
        Err("File must be a JSON file")
    }
}

pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    if parse(content) {
        println!("0")
    } else {
        println!("1")
    }
    Ok(())
}
