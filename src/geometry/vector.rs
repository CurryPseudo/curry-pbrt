use super::{Transform, Transformable};
use crate::def::{Float, Integer};
use nalgebra::base::dimension::{U2, U3};
pub type Vector<T, D> = nalgebra::VectorN<T, D>;
pub type Vector2f = nalgebra::VectorN<Float, U2>;
pub type Vector2i = nalgebra::VectorN<Integer, U2>;
pub type Vector3f = nalgebra::VectorN<Float, U3>;
pub type Vector3i = nalgebra::VectorN<Integer, U3>;

impl Transformable for Vector3f {
    fn apply(self, transform: &Transform) -> Self {
        Self::from_homogeneous(transform.m * self.to_homogeneous()).unwrap()
    }
}
