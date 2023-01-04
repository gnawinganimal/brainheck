
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
}
