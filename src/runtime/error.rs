use crate::Operation;

pub use ErrorKind::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    op: Operation,
}

impl Error {
    pub fn new(kind: ErrorKind, op: Operation) -> Self {
        Self {
            kind,
            op,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    IndexOutOfBounds,
    ReadError,
    WriteError,
}
