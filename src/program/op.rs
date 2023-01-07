
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Operation {
    AddPtr(u8),
    SubPtr(u8),
    AddCur(u8),
    SubCur(u8),
    Read,
    Write,
    Jump(usize),
    Back(usize),
}
