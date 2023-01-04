use std::{str::Chars, io::{Read, Write}};


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

    pub fn exec(&mut self, mut src: Chars) {
        while let Some(op) = src.next() {
            match op {
                '>' => self.mp += 1,
                '<' => self.mp -= 1,
                '+' => self.mem[self.mp] += 1,
                '-' => self.mem[self.mp] -= 1,
                '.' => {
                    self.writer.write(&[self.mem[self.mp]]).unwrap();
                },
                ',' => if let Some(b) = self.reader.bytes().next() {
                    self.mem[self.mp] = b.unwrap();
                },
                '[' => panic!("not implemented"),
                ']' => panic!("not implemented"),

                _ => (), // ignore non-instructions
            };
        };
    }
}
