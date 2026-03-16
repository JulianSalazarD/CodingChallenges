//! JSON Parser Library
//!
//! A lightweight JSON parser and validator library.

use std::error::Error;
use std::fs;

pub mod json;

use crate::json::json::{Json, JsonParser};

/// Builds and validates the file path from command-line arguments.
///
/// # Arguments
/// * `args` - An iterator over command-line arguments
///
/// # Returns
/// * `Ok(String)` - The validated file path ending with .json
/// * `Err(&'static str)` - Error message if validation fails
pub fn build(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let file_path = args.next().ok_or("No query provided")?;

    if file_path.ends_with(".json") {
        Ok(file_path)
    } else {
        Err("File must be a JSON file")
    }
}

/// Reads and validates a JSON file.
///
/// # Arguments
/// * `file_path` - Path to the JSON file to validate
///
/// # Returns
/// * `Ok(())` - If validation succeeds (prints 0)
/// * `Err(Box<dyn Error>)` - If file cannot be read or JSON is invalid (prints 1)
pub fn run(file_path: String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let mut json_object = JsonParser::new();
    json_object.build_json_object(content);

    match Json::build(&mut json_object) {
        Ok(_json) => {
            println!("0");
        }
        Err(_e) => {
            println!("1");
        }
    }

    Ok(())
}
