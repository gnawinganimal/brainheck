use crate::{Program, Op, Mem};
use std::io::{Read, Write};

pub struct Runtime<'a> {
    mem: Mem,

    reader: &'a mut dyn Read,
    writer: &'a mut dyn Write,

    ctrl_stack: Vec<usize>, // for control flow

    ip: usize, // program counter
}

impl<'a> Runtime<'a> {
    pub fn new(len: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            mem: Mem::new(len),
            ip: 0,

            reader,
            writer,

            ctrl_stack: vec![],
        }
    }

    pub fn exec(&mut self, prog: Program) -> Result<()> {
        loop {
            if let Some(op) = prog.get(self.ip) {
                match op {
                    Op::Next => self.mem.inc_ptr().unwrap(),
                    Op::Prev => self.mem.dec_ptr().unwrap(),
                    Op::Inc => self.mem.inc_cur().unwrap(),
                    Op::Dec => self.mem.dec_cur().unwrap(),
                    Op::Write => {
                        self.writer.write(&[self.mem.get_cur().unwrap()]).unwrap();
                    },
                    Op::Read => if let Some(b) = self.reader.bytes().next() {
                        self.mem.set_cur(b.unwrap()).unwrap();
                    },
                    Op::Skip => {
                        if self.mem.get_cur().unwrap() != 0 {
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
