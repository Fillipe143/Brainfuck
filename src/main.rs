use std::{fs, process::exit};
mod file;

fn main() {
    let file_path = file::get_file_path();
    if let Err(err) = file_path {
        throw_error(err);
        return;
    }

    let file_path = file_path.unwrap();

    let data = fs::read_to_string(file_path);
    if let Err(err) = data {
        throw_error(err.to_string());
        return;
    }

    let data = data.unwrap();
    println!("File contents: {}", data);
}

fn throw_error<'a>(err: String) {
    println!("Error: {}", err);
    exit(1);
}
