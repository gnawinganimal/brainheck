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

    fn get(&self, i: usize) -> Option<&u8> {
        if i < self.len {
            unsafe {
                Some(&*self.ptr.add(i))
            }
        } else {
            None
        }
    }

    fn set(&mut self, i: usize, b: u8) {
        unsafe {
            *self.ptr.add(i) = b;
        }
    }

    fn add(&mut self, i: usize, b: u8) {
        unsafe {
            *self.ptr.add(i) = (*self.ptr.add(i)).wrapping_add(b);
        }
    }

    fn sub(&mut self, i: usize, b: u8) {
        unsafe {
            *self.ptr.add(i) = (*self.ptr.add(i)).wrapping_sub(b);
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
