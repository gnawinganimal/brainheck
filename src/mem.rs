use std::{mem, alloc::{alloc, Layout, dealloc}};

pub struct Mem {
    ptr: *mut u8,
    len: usize,
}

impl Mem {
    pub fn new(len: usize) -> Self {
        Self { 
            ptr: unsafe {
                let layout = Layout::from_size_align_unchecked(len, mem::size_of::<u8>());
                alloc(layout)
            },
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> Option<u8> {
        if i < self.len {
            unsafe {
                Some(*self.ptr.add(i))
            }
        } else {
            None
        }
    }

    pub fn get_ref(&self, i: usize) -> Option<&u8> {
        if i < self.len {
            unsafe {
                Some(&*self.ptr.add(i))
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut u8> {
        if i < self.len {
            unsafe {
                Some(&mut *self.ptr.add(i))
            }
        } else {
            None
        }
    }
}

impl Drop for Mem {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr,
                Layout::from_size_align_unchecked(self.len, mem::size_of::<u8>())
            )
        };
    }
}
