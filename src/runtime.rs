use std::{io::{Read, Write}};

use crate::{prog::Prog, op::Op, mem::Mem};

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
            mp: 0,
            ip: 0,

            reader,
            writer,

            ctrl_stack: vec![],
        }
    }

    pub fn exec(&mut self, prog: Prog) {
        loop {
            if let Some(op) = prog.get(self.ip) {
                match op {
                    Op::Next => self.mem.next(),
                    Op::Prev => self.mem.prev(),
                    Op::Inc => self.mem.inc(),
                    Op::Dec => self.mem.dec(),
                    Op::Write => {
                        self.writer.write(&[self.mem.get().unwrap()]).unwrap();
                    },
                    Op::Read => if let Some(b) = self.reader.bytes().next() {
                        self.mem.set(b.unwrap());
                    },
                    Op::Skip => {
                        if self.mem.get().unwrap() != 0 {
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
        }
    }
}
