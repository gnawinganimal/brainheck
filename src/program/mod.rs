use std::{fs, str::FromStr, slice::Iter, fmt::{Display, Write}};

pub mod op;
pub mod error;

pub use op::Operation::{self, *};
pub use error::{Error, Result};

pub struct Program {
    inner: Vec<Operation>,
}

impl Program {
    pub fn from_file(path: String) -> Result<Self> {
        Self::try_from(fs::read_to_string(path).map_err(|_| Error::FileError)?)
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

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars().peekable();
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
                    inner.push(Jump(0));
                    continue;
                },
                ']' => {
                    if let Some(other) = stack.pop() {
                        *inner.get_mut(other).unwrap() = Jump(inner.len());
                        Back(other)
                    } else {
                        panic!("Matching bracket not found")
                    }
                },

                _ => continue,
            };

            inner.push(op);
        };

        Ok(Self {
            inner,
        })
    }
}

impl TryFrom<String> for Program {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_str(value.as_str())
    }
}

impl From<Vec<Operation>> for Program {
    fn from(inner: Vec<Operation>) -> Self {
        Self {
            inner,
        }
    }
}

impl From<Program> for Vec<Operation> {
    fn from(value: Program) -> Self {
        value.inner
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
        let program = Program::from_str("[->+<]").unwrap();
        let ops: Vec<Operation> = program.into();

        assert_eq!(ops, vec![Jump(5), SubCur(1), AddPtr(1), AddCur(1), SubPtr(1), Back(0)]);
    }

    #[test]
    fn encode_program() {
        let program = Program::from(vec![Jump(5), SubCur(1), AddPtr(1), AddCur(1), SubPtr(1), Back(0)]);
        let str = format!("{}", program);

        assert_eq!(str.as_str(), "[->+<]");
    }
}
