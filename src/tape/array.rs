use std::{mem, alloc::{Layout, alloc, dealloc}};

use super::Tape;

pub struct Array {
    ptr: *mut u8,
    len: usize,
}

impl Tape for Array {
    fn new(len: usize) -> Self {
        Self { 
            ptr: unsafe {
                let layout = Layout::from_size_align_unchecked(len, mem::size_of::<u8>());
                alloc(layout)
            },
            len,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, i: usize) -> Option<u8> {
        if i < self.len {
            unsafe {
                Some(*self.ptr.add(i))
            }
        } else {
            None
        }
    }

    fn set(&mut self, i: usize, b: u8) -> Option<u8> {
        if i < self.len {
            unsafe {
                let ptr = self.ptr.add(i);
                *ptr = b;
                Some(*ptr)
            }
        } else {
            None
        }
    }

    fn add(&mut self, i: usize, b: u8) -> Option<u8> {
        if i < self.len {
            unsafe {
                let ptr = self.ptr.add(i);
                *ptr = (*ptr).wrapping_add(b);
                Some(*ptr)
            }
        } else {
            None
        }
    }

    fn sub(&mut self, i: usize, b: u8) -> Option<u8> {
        if i < self.len {
            unsafe {
                let ptr = self.ptr.add(i);
                *ptr = (*ptr).wrapping_sub(b);
                Some(*ptr)
            }
        } else {
            None
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr,
                Layout::from_size_align_unchecked(self.len, mem::size_of::<u8>())
            )
        };
    }
}
