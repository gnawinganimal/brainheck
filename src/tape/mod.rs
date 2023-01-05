
pub mod array;

pub use array::Array;

type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IndexOutOfBounds,
}

pub trait Tape {
    fn new(len: usize) -> Self;

    fn len(&self) -> usize;

    fn get(&self, i: usize) -> Result<u8>;

    fn set(&mut self, i: usize, b: u8) -> Result<()>;
}
