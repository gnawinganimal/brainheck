use clap::Parser;
use std::io;

pub mod program;
pub mod runtime;
pub mod tape;

pub use program::{Program, Operation};
pub use runtime::Runtime;
pub use tape::Tape;

#[derive(Parser, Debug)]
#[command(author, version, about = "Brainfuck interpreter", long_about = None)]
struct Cli {
    path: String, // path to the brainfuck source

    #[arg(short = 'm', long = "mem", default_value_t = 30000)]
    mem_size: usize, // amount of memory to allocate
}

fn main() -> runtime::Result<()> {
    let Cli { path, mem_size } = Cli::parse();

    let program = Program::from_file(path).expect("Could not find source file");

    let mut i = io::stdin();
    let mut o = io::stdout();
    let mut rt: Runtime = Runtime::new(program, mem_size, &mut i, &mut o);

    rt.exec()
}
