use clap::Parser;
use colored::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;
use glob::Pattern;

/// Search for PATTERN in files
#[derive(Parser)]
struct Args {
    /// The text pattern to search for
    pattern: String,

    /// Files or glob patterns (e.g. *.rs)
    #[arg(required = true)]
    inputs: Vec<String>,

    /// Recurse into directories
    #[arg(short, long)]
    recursive: bool,
}

fn grep_file(pattern: &str, file_path: &PathBuf) {
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            if let Ok(line) = line {
                if line.contains(pattern) {
                    println!(
                        "{}:{}: {}",
                        file_path.display().to_string().green(),
                        (i + 1).to_string().yellow(),
                        line.replace(pattern, &pattern.red().bold().to_string())
                    );
                }
            }
        }
    } else {
        eprintln!("{}", format!("Could not open file {}", file_path.display()).red());
    }
}

fn collect_files(inputs: &[String], recursive: bool) -> Vec<PathBuf> {
    let mut files = vec![];

    for input in inputs {
        let pattern = Pattern::new(input).unwrap();

        if recursive {
            for entry in WalkDir::new(".").into_iter().flatten() {
                if entry.file_type().is_file() {
                    let path = entry.path().to_path_buf();
                    if pattern.matches_path(&path) {
                        files.push(path);
                    }
                }
            }
        } else {
            for entry in glob::glob(input).unwrap().flatten() {
                if entry.is_file() {
                    files.push(entry);
                }
            }
        }
    }

    files
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let files = collect_files(&args.inputs, args.recursive);

    if files.is_empty() {
        eprintln!("{}", "No files matched the given patterns.".red());
    } else {
        files.par_iter().for_each(|file| {
            grep_file(&args.pattern, file);
        });
    }

    Ok(())
}
