use std::fs;

pub fn build(mut args: impl Iterator<Item = String>) -> Result<String, String> {
    args.next(); // skip program name

    let file_path = args
        .next()
        .ok_or_else(|| "No file path provided".to_string())?;

    if file_path.is_empty() {
        return Err("No file path provided".to_string());
    }

    fs::read_to_string(&file_path).map_err(|e| e.to_string())
}
