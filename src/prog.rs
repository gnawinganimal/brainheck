
pub struct Prog {
    inner: Vec<Op>,
}

impl Prog {
    pub fn get(&self, i: usize) -> Option<Op> {
        self.inner.get(i).cloned()
    }
}

impl From<String> for Prog {
    fn from(src: String) -> Self {
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
            inner
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Op {
    Next,
    Prev,
    Add,
    Sub,
    Write,
    Read,
    Skip,
    Back,
}
