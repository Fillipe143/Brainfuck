use std::{env, path::Path};

pub fn get_file_path() -> Result<String, String> {
    let mut args = env::args();
    if args.len() < 2 {
        return Err("No path has passed".to_string());
    }

    let source_file = args.nth(1).unwrap();
    if let Some(err) = validate_file_path(source_file.clone()) {
        return Err(err);
    }

    Ok(source_file)
}

fn validate_file_path(file: String) -> Option<String> {
    let file_path = Path::new(file.as_str());

    if !file_path.exists() {
        return Some(format!("File '{}' does not exists", file));
    }

    if !file_path.is_file() {
        return Some(format!("'{}' must be a file", file));
    }

    if !file.ends_with(".bf") {
        return Some(format!("'{}' must be a Brainfuck extension", file));
    }

    None
}
