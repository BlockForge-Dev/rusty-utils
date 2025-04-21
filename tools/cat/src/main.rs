// tools/cat/src/main.rs
use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, Read};
use colored::*;

#[derive(Parser)]
struct Args {
    files: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    for filename in args.files {
        let file = File::open(&filename).map_err(|e| {
            eprintln!("{}", format!("Error opening {}: {}", filename, e).red());
            e
        })?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        print!("{}", content);
    }

    Ok(())
}
