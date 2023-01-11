use std::{mem, ops::{AddAssign, SubAssign}, alloc::{Layout, alloc, dealloc}};

pub struct Tape {
    len: usize,
    ptr: *mut u8, // base pointer
    ofs: usize, // current offset
}

impl Tape {
    pub fn new(len: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(len, mem::size_of::<u8>());
            alloc(layout)
        };

        Self { 
            ptr,
            ofs: 0,
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn raw(&self) -> *mut u8 {
        self.ptr.add(self.ofs)
    }

    pub fn get(&self) -> Option<u8> {
        unsafe {
            if self.ofs < self.len {
                Some(*self.raw())
            } else {
                None
            }
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut u8> {
        unsafe {
            if self.ofs < self.len {
                Some(&mut *self.raw())
            } else {
                None
            }
        }
    }

    pub fn insert(&mut self, b: u8) -> Option<()> {
        unsafe {
            if self.ofs < self.len {
                *self.raw() = b;
                Some(())
            } else {
                None
            }
        }
    }
}

impl AddAssign<usize> for Tape {
    fn add_assign(&mut self, rhs: usize) {
        self.ofs += rhs;
    }
}

impl SubAssign<usize> for Tape {
    fn sub_assign(&mut self, rhs: usize) {
        self.ofs -= rhs;
    }
}

impl Drop for Tape {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr,
                Layout::from_size_align_unchecked(self.len, mem::size_of::<u8>())
            )
        };
    }
}
