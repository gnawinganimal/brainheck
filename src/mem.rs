use std::{mem, alloc::{alloc, Layout, dealloc}};

pub struct Mem {
    inner: *mut u8,
    ptr: usize,
    len: usize,
}

impl Mem {
    pub fn new(len: usize) -> Self {
        Self { 
            inner: unsafe {
                let layout = Layout::from_size_align_unchecked(len, mem::size_of::<u8>());
                alloc(layout)
            },
            ptr: 0,
            len 
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn ptr(&self) -> usize {
        self.ptr
    }

    pub fn get(&self) -> Option<u8> {
        if self.ptr < self.len {
            unsafe {
                Some(*self.inner.add(self.ptr))
            }
        } else {
            None
        }
    }

    pub fn set(&mut self, b: u8) {
        if self.ptr < self.len {
            unsafe {
                *self.inner.add(self.ptr) = b;
            }
        }
    }

    pub fn prev(&mut self) {
        self.ptr -= 1;
    }

    pub fn next(&mut self) {
        self.ptr += 1;
    }

    pub fn inc(&mut self) {
        unsafe {
            *self.inner.add(self.ptr) += 1;
        }
    }

    pub fn dec(&mut self) {
        unsafe {
            *self.inner.add(self.ptr) -= 1;
        }
    }
}

impl Drop for Mem {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.inner,
                Layout::from_size_align_unchecked(self.len, mem::size_of::<u8>())
            )
        };
    }
}
