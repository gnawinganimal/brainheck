use std::fmt::Display;

pub use Operation::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Operation {
    AddPtr(u8),
    SubPtr(u8),
    AddCur(u8),
    SubCur(u8),
    Read,
    Write,
    Jump(usize),
    Back(usize),
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AddPtr(n) => ">".repeat(*n as usize),
            SubPtr(n) => "<".repeat(*n as usize),
            AddCur(n) => "+".repeat(*n as usize),
            SubCur(n) => "-".repeat(*n as usize),
            Read => ",".to_string(),
            Write => ".".to_string(),
            Jump(_) => "[".to_string(),
            Back(_) => "]".to_string(),
        };

        write!(f, "{}", s)
    }
}
