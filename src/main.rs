extern crate colored;

use colored::*;
use std::fs::ReadDir;
use std::io::Result;

fn main() {
    let files = read_current_dir();
    match files {
        Ok(files) => print_files(files),
        Err(_) => println!("Error reading directory"),
    }
}

fn read_current_dir() -> Result<ReadDir> {
    std::fs::read_dir(".")
}

fn print_files(files: ReadDir) {
    for file in files {
        if let Ok(file) = file {
            if file.file_type().unwrap().is_dir() {
                println!(
                    "{}",
                    file.path().display().to_string().truecolor(210, 168, 255)
                );
            } else {
                println!("{}", file.path().display());
            }
        }
    }
}
