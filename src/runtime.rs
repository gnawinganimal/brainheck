use crate::{Program, Op, Mem};
use std::io::{Read, Write};

pub struct Runtime<'a> {
    mem: Mem,
    mp: usize,
    ip: usize,

    reader: &'a mut dyn Read,
    writer: &'a mut dyn Write,

    ctrl_stack: Vec<usize>, // for control flow
}

impl<'a> Runtime<'a> {
    pub fn new(size: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            mem: Mem::new(size),
            ip: 0,
            mp: 0,

            reader,
            writer,

            ctrl_stack: vec![],
        }
    }

    pub fn exec(&mut self, prog: Program) -> Result<()> {
        loop {
            if let Some(op) = prog.get(self.ip) {
                match op {
                    Op::Next => self.mp += 1,
                    Op::Prev => self.mp -= 1,
                    Op::Inc => *self.mem.get_mut(self.mp).unwrap() += 1,
                    Op::Dec => *self.mem.get_mut(self.mp).unwrap() -= 1,
                    Op::Write => {
                        self.writer.write(&[self.mem.get(self.mp).unwrap()]).unwrap();
                    },
                    Op::Read => if let Some(b) = self.reader.bytes().next() {
                        *self.mem.get_mut(self.mp).unwrap() = b.unwrap();
                    },
                    Op::Skip => {
                        if self.mem.get(self.mp).unwrap() != 0 {
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

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    MemError,
}
