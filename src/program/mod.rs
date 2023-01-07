use std::slice::Iter;

pub mod op;

pub use op::Operation::{self, *};

pub struct Program {
    inner: Vec<Operation>,
}

impl Program {
    pub fn parse(src: String) -> Self {
        let mut chars = src.chars().peekable();
        let mut inner = Vec::new();

        let mut stack = Vec::new();

        while let Some(c) = chars.next() {
            println!("{}", c);
            let op = match c {
                '>' => {
                    let mut count = 1;
                    while let Some('>') = chars.peek() {
                        chars.next();
                        count += 1;
                    };
                    AddPtr(count)
                },
                '<' => {
                    let mut count = 1;
                    while let Some('<') = chars.peek() {
                        chars.next();
                        count += 1;
                    };
                    SubPtr(count)
                },
                '+' => {
                    let mut count = 1;
                    while let Some('+') = chars.peek() {
                        chars.next();
                        count += 1;
                    };
                    AddCur(count)
                },
                '-' => {
                    let mut count = 1;
                    while let Some('-') = chars.peek() {
                        chars.next();
                        count += 1;
                    };
                    SubCur(count)
                },
                '.' => Write,
                ',' => Read,
                '[' => {
                    println!("[ found!");
                    stack.push(inner.len());
                    continue;
                },
                ']' => {
                    println!("] found!");
                    if let Some(other) = stack.pop() {
                        inner.insert(other, Jump(inner.len()));
                        Back(other)
                    } else {
                        panic!("Matching bracket not found")
                    }
                },

                _ => continue,
            };

            inner.push(op);

            println!("Current program: {:?}", inner);
            println!("Current stack: {:?}", stack);
        };

        Self {
            inner,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, i: usize) -> Option<&Operation> {
        self.inner.get(i)
    }

    pub fn iter(&self) -> Iter<Operation> {
        self.inner.iter()
    }
}
