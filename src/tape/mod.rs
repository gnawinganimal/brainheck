
pub mod array;

pub use array::Array;

pub trait Tape {
    fn new(len: usize) -> Self;

    fn len(&self) -> usize;

    fn get(&self, i: usize) -> Option<u8>;

    fn set(&mut self, i: usize, b: u8) -> Option<u8>;

    fn add(&mut self, i: usize, b: u8) -> Option<u8>;

    fn sub(&mut self, i: usize, b: u8) -> Option<u8>;
}
