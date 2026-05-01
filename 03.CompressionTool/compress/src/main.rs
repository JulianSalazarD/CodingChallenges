mod codec;

use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(name = "compress")]
#[command(about = "A simple file compression tool")]
struct Cli {
    /// Input file to process
    filename: String,
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new(&cli.filename);

    if !path.exists() {
        eprintln!("Error: file '{}' does not exist", cli.filename);
        std::process::exit(1);
    }

    if !path.is_file() {
        eprintln!("Error: '{}' is not a regular file", cli.filename);
        std::process::exit(1);
    }

    println!("Processing file: {}", cli.filename);
}
