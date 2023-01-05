use std::{mem, alloc::{Layout, alloc, dealloc}};

use super::{Tape, Result, Error};

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

    fn get(&self, i: usize) -> Result<u8> {
        if i < self.len {
            unsafe {
                Ok(*self.ptr.add(i))
            }
        } else {
            Err(Error::IndexOutOfBounds)
        }
    }

    fn set(&mut self, i: usize, b: u8) -> Result<()> {
        if i < self.len {
            unsafe {
                Ok(*self.ptr.add(i) = b)
            }
        } else {
            Err(Error::IndexOutOfBounds)
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
