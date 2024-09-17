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
            println!("{}", file.path().display());
        }
    }
}
