
use std::fmt::Display;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Op {
    Next,
    Prev,
    Inc,
    Dec,
    Write,
    Read,
    Skip,
    Back,
}

impl From<Op> for char {
    fn from(value: Op) -> Self {
        match value {
            Op::Next => '>',
            Op::Prev => '<',
            Op::Inc => '+',
            Op::Dec => '-',
            Op::Write => '.',
            Op::Read => ',',
            Op::Skip => '[',
            Op::Back => ']',
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
