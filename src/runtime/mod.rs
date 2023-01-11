use crate::{Program, program::op::*, Tape};
use std::{io::{Read, Write}};

pub mod error;

pub use error::{Error, Result};

pub struct Runtime<'a> {
    tape: Tape,

    reader: &'a mut dyn Read,
    writer: &'a mut dyn Write,

    ip: usize, // program counter
}

impl<'a> Runtime<'a> {
    pub fn new(mem_len: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            tape: Tape::new(mem_len),

            reader,
            writer,

            ip: 0,
        }
    }

    pub fn exec(&mut self, prog: Program) -> Result<()> {
        loop {
            if let Some(op) = prog.get(self.ip) {
                match op {
                    AddPtr(n) => self.tape += *n,
                    SubPtr(n) => self.tape -= *n,
                    AddCur(n) => *self.tape.get_mut().ok_or(Error::IndexOutOfBounds)? += *n,
                    SubCur(n) => *self.tape.get_mut().ok_or(Error::IndexOutOfBounds)? -= *n,
                    Write => if let Some(b) = self.tape.get() {
                        self.writer.write(&[b]).map_err(|_| Error::WriteError)?;
                    } else {
                        return Err(Error::IndexOutOfBounds)
                    },
                    Read => if let Some(Ok(b)) = self.reader.bytes().next() {
                        *self.tape.get_mut().ok_or(Error::IndexOutOfBounds)? = b;
                    } else {
                        return Err(Error::ReadError)
                    },
                    Jump(n) => if let Some(b) = self.tape.get() {
                        if b == 0 {
                            self.ip = *n;
                        }
                    } else {
                        return Err(Error::IndexOutOfBounds)
                    },
                    Back(n) => if let Some(b) = self.tape.get() {
                        if b != 0 {
                            self.ip = *n;
                        }
                    } else {
                        return Err(Error::IndexOutOfBounds)
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
        Runtime::new(30000, &mut reader.as_slice(), &mut writer).exec(pr).expect("Program quit unexpectedly");
        assert_eq!(std::str::from_utf8(&writer).unwrap(), "Hello World!\n");
    }
}
