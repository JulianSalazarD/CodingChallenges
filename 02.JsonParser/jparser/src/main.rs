//! JSON Parser CLI Application
//!
//! This is the command-line interface for the JSON parser.
//! It accepts a JSON file path as argument and validates it.

use jparser::{build, run};
use std::{env, process};

/// Main entry point for the application.
/// Parses command-line arguments and runs the JSON validator.
fn main() {
    let file_path = build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(file_path).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}
