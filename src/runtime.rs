use crate::{Program, Op, Tape, tape};
use std::io::{self, Read, Write};

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

    ctrl_stack: Vec<usize>, // for control flow

    ip: usize, // program counter
}

impl<'a, T: Tape> Runtime<'a, T> {
    pub fn new(len: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            tape: Tape::new(len),
            tp: 0,

            reader,
            writer,

            ctrl_stack: vec![],

            ip: 0,
        }
    }

    pub fn get_cur(&self) -> Result<u8> {
        self.tape.get(self.tp).map_err(|e| Error::IndexOutOfBounds)
    }

    pub fn set_cur(&mut self, b: u8) -> Result<()> {
        self.tape.set(self.tp, b).map_err(|e| Error::IndexOutOfBounds)
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
                    Op::Next => self.tp += 1,
                    Op::Prev => self.tp -= 1,
                    Op::Inc =>  self.add_cur(1)?,
                    Op::Dec =>  self.sub_cur(1)?,
                    Op::Write => {
                        self.writer.write(&[self.get_cur()?])
                            .map_err(|_| Error::Write)?;
                    },
                    Op::Read => if let Some(Ok(b)) = self.reader.bytes().next() {
                        self.set_cur(b)?;
                    },
                    Op::Skip => {
                        if self.get_cur()? != 0 {
                            self.ctrl_stack.push(self.ip);
                        } else {
                            let mut count = 0;
                            'ctrl: loop {
                                self.ip += 1;
                                if let Some(op) = prog.get(self.ip) {
                                    match op {
                                        Op::Skip => count += 1,
                                        Op::Back => {
                                            if count == 0 {
                                                break 'ctrl;
                                            }
                                            count -= 1;
                                        },
                                        _ => (),
                                    }
                                } else {
                                    break 'ctrl;
                                }
                            }
                        }
                    },
                    Op::Back => {
                        self.ip = self.ctrl_stack.pop().unwrap() - 1;
    
                    },
                };
    
                self.ip += 1;
            }
        };
    }
}
