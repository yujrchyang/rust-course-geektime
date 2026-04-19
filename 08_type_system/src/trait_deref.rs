use std::ops::{Deref, DerefMut};

fn main() {
    let mut buf = Buffer::new(vec![1, 4, 2, 3]);
    buf.sort_unstable();
    println!("buf: {:?}", buf);
}

#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}
