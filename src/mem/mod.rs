
pub mod array;

pub use array::Array;

pub struct Mem {
    arr: Array<u8>,
    cur: usize,
}

impl Mem {
    pub fn new(len: usize) -> Self {
        Self {
            arr: Array::new(len),
            cur: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn cur(&self) -> usize {
        self.cur
    }

    pub fn inc_ptr(&mut self) -> Result<()> {
        self.cur += 1;
        Ok(())
    }

    pub fn dec_ptr(&mut self) -> Result<()> {
        self.cur -= 1;
        Ok(())
    }

    pub fn get_cur(&self) -> Result<u8> {
        match self.arr.get(self.cur) {
            Some(b) => Ok(*b),
            None => Err(Error::InvalidIdx),
        }
    }

    pub fn set_cur(&mut self, v: u8) -> Result<()> {
        match self.arr.get_mut(self.cur) {
            Some(b) => Ok(*b = v),
            None => Err(Error::InvalidIdx),
        }
    }

    pub fn inc_cur(&mut self) -> Result<()> {
        if let Some(b) = self.arr.get_mut(self.cur) {
            *b += 1;
            Ok(())
        } else {
            Err(Error::InvalidIdx)
        }
    }

    pub fn dec_cur(&mut self) -> Result<()> {
        if let Some(b) = self.arr.get_mut(self.cur) {
            *b -= 1;
            Ok(())
        } else {
            Err(Error::InvalidIdx)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    InvalidPtr,
    InvalidIdx,
}
