
use std::{fmt::Display, slice::Iter};

use crate::op::Op;

pub struct Prog {
    inner: Vec<Op>,
}

impl Prog {
    pub fn get(&self, i: usize) -> Option<Op> {
        self.inner.get(i).cloned()
    }

    pub fn iter(&self) -> Iter<Op> {
        self.inner.iter()
    }
}

impl From<&str> for Prog {
    fn from(src: &str) -> Self {
        let mut inner = vec![];

        for op_c in src.chars() {
            match op_c {
                '>' => inner.push(Op::Next),
                '<' => inner.push(Op::Prev),
                '+' => inner.push(Op::Inc),
                '-' => inner.push(Op::Dec),
                '.' => inner.push(Op::Write),
                ',' => inner.push(Op::Read),
                '[' => inner.push(Op::Skip),
                ']' => inner.push(Op::Back),

                _ => (),
            };
        }

        Self {
            inner,
        }
    }
}

impl From<String> for Prog {
    fn from(src: String) -> Self {
        src.as_str().into()
    }
}

impl From<&Prog> for String {
    fn from(value: &Prog) -> Self {
        let mut s = String::new();

        for op in value.iter() {
            s.push(char::from(*op))
        };

        s
    }
}

impl Display for Prog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
