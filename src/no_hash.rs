use std::{hash::Hasher, marker::PhantomData};

#[derive(Debug, Copy)]
pub struct NoHash<T>(u64, PhantomData<T>);

impl<T> Default for NoHash<T> {
    fn default() -> Self {
        NoHash(0, PhantomData)
    }
}

impl<T> Clone for NoHash<T> {
    fn clone(&self) -> Self {
        NoHash(self.0, self.1)
    }
}

impl<T> Hasher for NoHash<T> {
    fn write(&mut self, ns: &[u8]) {
        for n in ns {
            self.write_u8(*n);
        }
    }

    fn write_u8(&mut self, n: u8) {
        self.0 = u64::from(n)
    }
    fn write_u16(&mut self, n: u16) {
        self.0 = u64::from(n)
    }
    fn write_u32(&mut self, n: u32) {
        self.0 = u64::from(n)
    }
    fn write_u64(&mut self, n: u64) {
        self.0 = n
    }
    fn write_usize(&mut self, n: usize) {
        self.0 = n as u64
    }
    fn write_i8(&mut self, n: i8) {
        self.0 = n as u64
    }
    fn write_i16(&mut self, n: i16) {
        self.0 = n as u64
    }
    fn write_i32(&mut self, n: i32) {
        self.0 = n as u64
    }
    fn write_i64(&mut self, n: i64) {
        self.0 = n as u64
    }
    fn write_isize(&mut self, n: isize) {
        self.0 = n as u64
    }

    fn finish(&self) -> u64 {
        self.0
    }
}
