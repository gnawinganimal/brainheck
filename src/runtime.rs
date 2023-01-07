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

    pub fn exec(&mut self, prog: Program) -> Result<()> {
        loop {
            if let Some(op) = prog.get(self.ip) {
                match op {
                    AddPtr(n) => self.tp = self.tp.wrapping_add(*n),
                    SubPtr(n) => self.tp = self.tp.wrapping_sub(*n),
                    AddCur(n) => self.tape.add(self.tp, *n),
                    SubCur(n) => self.tape.sub(self.tp, *n),
                    Write => {
                        self.writer.write(&[*self.tape.get(self.tp).unwrap()])
                            .map_err(|_| Error::Write)?;
                    },
                    Read => if let Some(Ok(b)) = self.reader.bytes().next() {
                        self.tape.set(self.tp, b);
                    },
                    Jump(n) => {
                        if *self.tape.get(self.tp).unwrap() == 0 {
                            self.ip = *n + 1;
                        }
                    },
                    Back(n) => {
                        if *self.tape.get(self.tp).unwrap() != 0 {
                            self.ip = *n + 1;
                        }
                    },
                };
    
                self.ip += 1;
            } else {
                break Ok(())
            }
        }
    }
}
