use crate::def::{Float, Integer};
use nalgebra::base::dimension::{U2, U3};
pub type Vector<T, D> = nalgebra::VectorN<T, D>;
pub type Vector2f = nalgebra::VectorN<Float, U2>;
pub type Vector2i = nalgebra::VectorN<Integer, U2>;
pub type Vector3f = nalgebra::VectorN<Float, U3>;
pub type Vector3i = nalgebra::VectorN<Integer, U3>;
