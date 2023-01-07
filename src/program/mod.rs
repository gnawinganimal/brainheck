use std::{slice::Iter, fmt::{Display, Write}};

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
                    stack.push(inner.len());
                    continue;
                },
                ']' => {
                    if let Some(other) = stack.pop() {
                        inner.insert(other, Jump(inner.len() + 1));
                        Back(other)
                    } else {
                        panic!("Matching bracket not found")
                    }
                },

                _ => continue,
            };

            inner.push(op);
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

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for op in self.iter() {
            write!(s, "{}", op)?;
        }

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_program() {
        let program = Program::parse(String::from("[->+<]"));
        let ops: Vec<_> = program.iter().copied().collect();

        assert_eq!(ops, vec![Jump(5), SubCur(1), AddPtr(1), AddCur(1), SubPtr(1), Back(0)])
    }
}
