use crate::{Program, Operation::*, Tape};
use std::io::{Read, Write};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IndexOutOfBounds,
    Read,
    Write,
}

pub struct Runtime<'a, T: Tape> {
    tape: T,
    tp: usize,

    reader: &'a mut dyn Read,
    writer: &'a mut dyn Write,

    ip: usize, // program counter
}

impl<'a, T: Tape> Runtime<'a, T> {
    pub fn new(len: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            tape: Tape::new(len),
            tp: 0,

            reader,
            writer,

            ip: 0,
        }
    }

    pub fn get_cur(&self) -> Result<u8> {
        self.tape.get(self.tp).map_err(|_| Error::IndexOutOfBounds)
    }

    pub fn set_cur(&mut self, b: u8) -> Result<()> {
        self.tape.set(self.tp, b).map_err(|_| Error::IndexOutOfBounds)
    }

    pub fn add_cur(&mut self, b: u8) -> Result<()> {
        self.set_cur(self.get_cur()? + b)
    }

    pub fn sub_cur(&mut self, b: u8) -> Result<()> {
        self.set_cur(self.get_cur()? - b)
    }

    pub fn exec(&mut self, prog: Program) -> Result<()> {
        loop {
            if let Some(op) = prog.get(self.ip) {
                match op {
                    AddPtr(n) => self.tp += *n as usize,
                    SubPtr(n) => self.tp -= *n as usize,
                    AddCur(n) =>  self.add_cur(*n)?,
                    SubCur(n) =>  self.sub_cur(*n)?,
                    Write => {
                        self.writer.write(&[self.get_cur()?])
                            .map_err(|_| Error::Write)?;
                    },
                    Read => if let Some(Ok(b)) = self.reader.bytes().next() {
                        self.set_cur(b)?;
                    },
                    Jump(n) => {
                        if self.get_cur()? != 0 {
                            
                        } else {
                            self.ip = *n;
                        }
                    },
                    Back(n) => {
                        if self.get_cur()? == 0 {
                            
                        } else {
                            self.ip = *n;
                        }
                    },
                };
    
                self.ip += 1;
            }
        };
    }
}
