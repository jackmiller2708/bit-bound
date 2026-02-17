pub struct FixedPool<T, const N: usize> {
    items: [T; N],
    len: usize,
}

impl<T: Copy, const N: usize> FixedPool<T, N> {
    pub const fn new(default: T) -> Self {
        Self {
            items: [default; N],
            len: 0,
        }
    }

    pub fn spawn(&mut self, item: T) -> Result<(), ()> {
        if self.len >= N {
            return Err(());
        }

        self.items[self.len] = item;
        self.len += 1;
        Ok(())
    }

    pub fn despawn(&mut self, index: usize) {
        if index >= self.len {
            return;
        }

        self.len -= 1;
        self.items[index] = self.items[self.len];
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.items[..self.len]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        N
    }
}
