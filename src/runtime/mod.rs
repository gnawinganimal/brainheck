use crate::{Program, program::op::*, Tape};
use std::{io::{Read, Write}};

pub mod error;

pub use error::{Error, Result};

pub struct Runtime<'a> {
    program: Program,

    tape: Tape,

    reader: &'a mut dyn Read,
    writer: &'a mut dyn Write,

    pc: usize, // program counter
}

impl<'a> Runtime<'a> {
    pub fn new(program: Program, mem_len: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            program,

            tape: Tape::new(mem_len),

            reader,
            writer,

            pc: 0,
        }
    }

    pub fn next(&mut self) -> Option<Result<()>> {
        let op = self.program.get(self.pc)?;

        match op {
            AddPtr(n) => self.tape += *n,
            SubPtr(n) => self.tape -= *n,
            AddCur(n) => if let Some(b) = self.tape.get_mut() {
                *b += *n;
            } else {
                return Some(Err(Error::IndexOutOfBounds));
            },
            SubCur(n) => if let Some(b) = self.tape.get_mut() {
                *b -= *n;
            } else {
                return Some(Err(Error::IndexOutOfBounds));
            },
            Write => if let Some(b) = self.tape.get() {
                if let Err(_) = self.writer.write(&[b]) {
                    return Some(Err(Error::WriteError));
                };
            } else {
                return Some(Err(Error::IndexOutOfBounds));
            },
            Read => if let Some(Ok(i)) = self.reader.bytes().next() {
                if let Some(b) = self.tape.get_mut() {
                    *b = i;
                } else {
                    return Some(Err(Error::IndexOutOfBounds));
                }
            } else {
                return Some(Err(Error::ReadError));
            },
            Jump(n) => if let Some(b) = self.tape.get() {
                if b == 0 {
                    self.pc = *n;
                }
            } else {
                return Some(Err(Error::IndexOutOfBounds))
            },
            Back(n) => if let Some(b) = self.tape.get() {
                if b != 0 {
                    self.pc = *n;
                }
            } else {
                return Some(Err(Error::IndexOutOfBounds))
            },
        };

        self.pc += 1;
        Some(Ok(()))
    }

    pub fn exec(&mut self) -> Result<()> {
        while let Some(res) = self.next() {
            match res {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        };

        Ok(())
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

        let prog = Program::from_file("bf/hello_world.bf".to_string()).expect("Could not find fb/hello_world.bf");
        Runtime::new(prog, 30000, &mut reader.as_slice(), &mut writer).exec().expect("Program quit unexpectedly");
        assert_eq!(std::str::from_utf8(&writer).unwrap(), "Hello World!\n");
    }
}
