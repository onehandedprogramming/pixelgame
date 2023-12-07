use std::ops::{Deref, DerefMut};

pub struct SwapBuffer<T> {
    pub read: Vec<T>,
    pub write: Vec<T>,
}

impl<T: Default + Clone> SwapBuffer<T> {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.read, &mut self.write);
    }

    pub fn new(size: usize) -> Self {
        Self {
            read: vec![T::default(); size],
            write: vec![T::default(); size],
        }
    }
}

impl<T> Deref for SwapBuffer<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.read
    }
}

impl<T> DerefMut for SwapBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.write
    }
}
