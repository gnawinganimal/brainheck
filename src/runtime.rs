use std::{io::{Read, Write}};

use crate::{prog::Prog, op::Op};

pub struct Runtime<'a> {
    mem: Vec<u8>,
    mp: usize,
    ip: usize,

    reader: &'a mut dyn Read,
    writer: &'a mut dyn Write,

    ctrl_stack: Vec<usize>, // for control flow
}

impl<'a> Runtime<'a> {
    pub fn new(size: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            mem: vec![0; size],
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
                    Op::Next => self.mp += 1,
                    Op::Prev => self.mp -= 1,
                    Op::Add => self.mem[self.mp] += 1,
                    Op::Sub => self.mem[self.mp] -= 1,
                    Op::Write => {
                        self.writer.write(&[self.mem[self.mp]]).unwrap();
                    },
                    Op::Read => if let Some(b) = self.reader.bytes().next() {
                        self.mem[self.mp] = b.unwrap();
                    },
                    Op::Skip => {
                        if self.mem[self.mp] != 0 {
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
