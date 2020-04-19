mod bounds;
mod normal;
mod point;
mod ray;
mod shape;
mod transform;
mod vector;
use crate::*;
pub use bounds::*;
pub use normal::*;
pub use point::*;
pub use ray::*;
pub use shape::*;
use std::ops::Index;
pub use transform::*;
pub use vector::*;

pub trait Permutable<T>: Index<usize, Output = T> {
    fn permute_new(x: &T, y: &T, z: &T) -> Self;
}

pub fn permute<R: Permutable<T>, T>(p: R, x: usize, y: usize, z: usize) -> R {
    R::permute_new(&p[x], &p[y], &p[z])
}

impl Permutable<Float> for Point3f {
    fn permute_new(x: &Float, y: &Float, z: &Float) -> Self {
        Self::new(*x, *y, *z)
    }
}
impl Permutable<Float> for Vector3f {
    fn permute_new(x: &Float, y: &Float, z: &Float) -> Self {
        Self::new(*x, *y, *z)
    }
}
