// tools/echo/src/main.rs
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input text to echo
    input: Vec<String>,

    /// Do not output the trailing newline
    #[arg(short = 'n', long)]
    no_newline: bool,
}

fn main() {
    let args = Args::parse();
    let output = args.input.join(" ");
    if args.no_newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }
}
