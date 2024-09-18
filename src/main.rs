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
        Ok(files) => {
            for file in files {
                if let Ok(file) = file {
                    read_entry(file, &config);
                }
            }
            print!("\n")
        }
        Err(_) => {}
    }
}

fn read_current_dir() -> Result<ReadDir> {
    std::fs::read_dir(".")
}

fn is_hidden(entry: &DirEntry) -> bool {
    let file_name = entry.file_name();
    file_name
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn read_entry(file: DirEntry, config: &Config) {
    if is_hidden(&file) && !config.all {
        return;
    }
    print!("{}\t", file.file_name().to_str().unwrap());
}
