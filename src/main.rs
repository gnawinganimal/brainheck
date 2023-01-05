use clap::Parser;
use std::{fs, io};

pub mod program;
pub mod runtime;
pub mod mem;

pub use program::{Program, Op};
pub use runtime::Runtime;
pub use mem::Mem;

#[derive(Parser, Debug)]
#[command(author, version, about = "Brainfuck interpreter", long_about = None)]
struct Cli {
    path: String, // path to the brainfuck source

    #[arg(short = 'm', long = "mem", default_value_t = 30000)]
    mem_size: usize, // amount of memory to allocate
}

fn main() {
    let Cli { path, mem_size } = Cli::parse();

    let program = Program::from(fs::read_to_string(path).expect("Could not find source file"));

    let mut i = io::stdin();
    let mut o = io::stdout();
    let mut rt = Runtime::new(mem_size, &mut i, &mut o);

    rt.exec(program);
}
