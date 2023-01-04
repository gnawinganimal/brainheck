
use std::fmt::Display;

use crate::op::Op;

pub struct Prog {
    inner: Vec<Op>,
}

impl Prog {
    pub fn get(&self, i: usize) -> Option<Op> {
        self.inner.get(i).cloned()
    }
}

impl From<&str> for Prog {
    fn from(src: &str) -> Self {
        let mut inner = vec![];

        for op_c in src.chars() {
            match op_c {
                '>' => inner.push(Op::Next),
                '<' => inner.push(Op::Prev),
                '+' => inner.push(Op::Add),
                '-' => inner.push(Op::Sub),
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

        for op in value.inner.iter() {
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
