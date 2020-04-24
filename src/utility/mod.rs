use crate::*;
use std::ops::Index;
use std::ops::IndexMut;
#[derive(Debug)]
pub struct FixedVec2D<T> {
    row_size: usize,
    vec: Vec<T>,
}

impl<T: Clone> FixedVec2D<T> {
    pub fn new(t: T, size: Vector2u) -> Self {
        Self {
            row_size: size.x,
            vec: vec![t; size.x * size.y],
        }
    }
    pub fn from_vec(vec: Vec<T>, row_size: usize) -> Self {
        Self { row_size, vec }
    }
}

impl<T> FixedVec2D<T> {
    pub fn size(&self) -> Vector2u {
        Vector2u::new(self.row_size, self.vec.len() / self.row_size)
    }
}

impl<T> Index<Point2u> for FixedVec2D<T> {
    fn index(&self, i: Point2u) -> &<Self as Index<Point2u>>::Output {
        &self.vec[i.x + i.y * self.row_size]
    }
    type Output = T;
}

impl<T> IndexMut<Point2u> for FixedVec2D<T> {
    fn index_mut(&mut self, i: Point2u) -> &mut <Self as Index<Point2u>>::Output {
        &mut self.vec[i.x + i.y * self.row_size]
    }
}

impl<T> AsRef<[T]> for FixedVec2D<T> {
    fn as_ref(&self) -> &[T] {
        &self.vec
    }
}
impl<T> AsMut<[T]> for FixedVec2D<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.vec
    }
}

impl<T> IntoIterator for FixedVec2D<T> {
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.vec.into_iter()
    }
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;
}
