use std::{mem, alloc::{Layout, alloc, dealloc}};

pub struct Array<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> Array<T> {
    pub fn new(len: usize) -> Self {
        Self { 
            ptr: unsafe {
                let layout = Layout::from_size_align_unchecked(len, mem::size_of::<T>());
                alloc(layout) as *mut T
            },
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.len {
            unsafe {
                Some(&*self.ptr.add(i))
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < self.len {
            unsafe {
                Some(&mut *self.ptr.add(i))
            }
        } else {
            None
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align_unchecked(self.len, mem::size_of::<T>())
            )
        };
    }
}
