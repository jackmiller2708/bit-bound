use std::mem::{align_of, size_of};

pub struct Arena<const SIZE: usize> {
    buffer: [u8; SIZE],
    offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    OutOfMemory,
}

impl<const SIZE: usize> Arena<SIZE> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; SIZE],
            offset: 0,
        }
    }

    fn align_up(addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }

    pub fn alloc<T>(&mut self, value: T) -> Result<&mut T, MemoryError> {
        let align = align_of::<T>();
        let size = size_of::<T>();

        let start = Self::align_up(self.offset, align);
        let end = start + size;

        if end > SIZE {
            return Err(MemoryError::OutOfMemory);
        }

        self.offset = end;

        let ptr = self.buffer.as_mut_ptr();

        Ok(unsafe {
            let typed_ptr = ptr.add(start) as *mut T;
            typed_ptr.write(value);
            &mut *typed_ptr
        })
    }

    pub fn alloc_slice<T>(&mut self, count: usize) -> Result<&mut [T], MemoryError> {
        let align = align_of::<T>();
        let size = size_of::<T>() * count;

        let start = Self::align_up(self.offset, align);
        let end = start + size;

        if end > SIZE {
            return Err(MemoryError::OutOfMemory);
        }

        self.offset = end;

        let ptr = self.buffer.as_mut_ptr();

        Ok(unsafe {
            let typed_ptr = ptr.add(start) as *mut T;
            std::slice::from_raw_parts_mut(typed_ptr, count)
        })
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn used(&self) -> usize {
        self.offset
    }

    pub fn remaining(&self) -> usize {
        SIZE - self.offset
    }
}

pub struct RuntimeMemory {
    pub global: Arena<{ 256 * 1024 }>,
    pub level: Arena<{ 512 * 1024 }>,
    pub frame: Arena<{ 256 * 1024 }>,
}

impl RuntimeMemory {
    pub fn new() -> Self {
        Self {
            global: Arena::new(),
            level: Arena::new(),
            frame: Arena::new(),
        }
    }
}
