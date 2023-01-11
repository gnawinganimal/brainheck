use crate::{Program, Operation::{*, self}, Tape};
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
    pub fn new(mem_len: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            tape: Tape::new(mem_len),
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
                    AddCur(n) => {
                        self.tape.add(self.tp, *n).ok_or(Error::IndexOutOfBounds)?;
                    },
                    SubCur(n) => {
                        self.tape.sub(self.tp, *n).ok_or(Error::IndexOutOfBounds)?;
                    },
                    Write => {
                        if let Some(b) = self.tape.get(self.tp) {
                            self.writer.write(&[b])
                                .map_err(|_| Error::Write)?;
                        } else {
                            return Err(Error::IndexOutOfBounds)
                        }
                    },
                    Read => if let Some(Ok(b)) = self.reader.bytes().next() {
                        self.tape.set(self.tp, b);
                    },
                    Jump(n) => {
                        if let Some(b) = self.tape.get(self.tp) {
                            if b == 0 {
                                self.ip = *n;
                            }
                        } else {
                            return Err(Error::IndexOutOfBounds)
                        }
                    },
                    Back(n) => {
                        if let Some(b) = self.tape.get(self.tp) {
                            if b != 0 {
                                self.ip = *n;
                            }
                        } else {
                            return Err(Error::IndexOutOfBounds)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn hello_world() {
        let reader = Vec::new();
        let mut writer = Vec::new();

        let pr = Program::from_file("bf/hello_world.bf".to_string()).expect("Could not find fb/hello_world.bf");
        Runtime::<tape::Array>::new(30000, &mut reader.as_slice(), &mut writer).exec(pr).expect("Program quit unexpectedly");
        assert_eq!(std::str::from_utf8(&writer).unwrap(), "Hello World!\n");
    }
}
