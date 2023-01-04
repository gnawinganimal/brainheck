use std::str::Chars;


pub struct Runtime {
    mem: Vec<u8>,
    mp: usize,
    ip: usize,

    addr_stack: Vec<usize>, // might not need
}

impl Runtime {
    pub fn new(size: usize) -> Self {
        Self {
            mem: vec![0; size],
            mp: 0,
            ip: 0,

            addr_stack: vec![],
        }
    }

    pub fn exec(&mut self, mut src: Chars) {
        while let Some(op) = src.next() {
            match op {
                '>' => self.mp += 1,
                '<' => self.mp -= 1,
                '+' => self.mem[self.mp] += 1,
                '-' => self.mem[self.mp] -= 1,
                '.' => panic!("not implemented"),
                ',' => panic!("not implemented"),
                '[' => panic!("not implemented"),
                ']' => panic!("not implemented"),

                _ => (), // ignore non-instructions
            }
        }
    }
}
