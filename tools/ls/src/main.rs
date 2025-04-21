// tools/ls/src/main.rs
use clap::Parser;
use std::fs;
use colored::*;

#[derive(Parser)]
struct Args {
    #[arg(default_value = ".")]
    path: String,
}

fn main() {
    let args = Args::parse();

    match fs::read_dir(&args.path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let file_name = entry.file_name().into_string().unwrap_or_default();
                println!("{}", file_name.blue());
            }
        }
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
    }
}
