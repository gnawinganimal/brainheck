
use clap::Parser;
use std::fs;

use crate::runtime::Runtime;

pub mod runtime;

#[derive(Parser, Debug)]
#[command(author = "Evelyn Heller", version, about = "Brainfuck interpreter", long_about = None)]
struct Cli {
    path: String, // path to the brainfuck source

    #[arg(short = 'm', long = "mem", default_value_t = 30000)]
    mem_size: usize, // amount of memory to allocate
}

fn main() {
    let Cli { path, mem_size } = Cli::parse();

    let _ = fs::read_to_string(path).expect("Could not read file");
    let _ = Runtime::new(mem_size);
}
