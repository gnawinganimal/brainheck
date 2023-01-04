
use clap::Parser;
use std::fs;

pub mod runtime;

#[derive(Parser, Debug)]
#[command(author = "Evelyn Heller", version, about = "Brainfuck interpreter", long_about = None)]
struct Cli {
    path: String,
}

fn main() {
    let Cli { path } = Cli::parse();

    let src = fs::read_to_string(path).expect("Could not read file");

    println!("{}", src);
}
