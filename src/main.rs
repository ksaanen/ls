extern crate colored;

use colored::*;
use std::fs::{DirEntry, ReadDir};
use std::io::Result;

struct Config {
    all: bool,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let mut all = false;

        for arg in args {
            if arg == "-a" {
                all = true;
            }
        }

        Config { all }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let config = Config::new(&args);

    let files = read_current_dir();
    match files {
        Ok(files) => print_files(files, &config),
        Err(_) => println!("Error reading directory"),
    }
}

fn read_current_dir() -> Result<ReadDir> {
    std::fs::read_dir(".")
}

fn print_files(files: ReadDir, config: &Config) {
    for file in files {
        if let Ok(file) = file {
            if is_hidden(&file) && !config.all {
                continue;
            }
            let file_medatada = file.metadata();

            if let Ok(file_medatada) = file_medatada {
                if file_medatada.is_symlink() {
                    // Check
                    println!("{}", file.path().display().to_string().truecolor(255, 0, 0));
                }
                if file_medatada.is_file() {
                    // Check
                    println!("{}", file.path().display().to_string().truecolor(0, 255, 0));
                }
                if file_medatada.is_dir() {
                    // Check
                    println!(
                        "{}",
                        file.path().display().to_string().truecolor(210, 168, 255)
                    );
                }
            }
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    let file_name = entry.file_name();
    file_name
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
