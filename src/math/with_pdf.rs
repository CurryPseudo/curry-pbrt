use crate::def::Float;
use std::ops::{Deref, DerefMut};

pub struct WithPdf<T> {
    pub t: T,
    pub pdf: Float,
}

impl<T> WithPdf<T> {
    pub fn new(t: T, pdf: Float) -> Self {
        Self { t, pdf }
    }
}
impl<T> Deref for WithPdf<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for WithPdf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<T> WithPdf<T> {
    pub fn replace<R>(self, r: R) -> WithPdf<R> {
        WithPdf::new(r, self.pdf)
    }
    pub fn map<R, F: Fn(T) -> R>(self, f: F) -> WithPdf<R> {
        WithPdf::new(f(self.t), self.pdf)
    }
}
