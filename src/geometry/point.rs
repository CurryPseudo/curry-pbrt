use crate::def::Integer;
use crate::def::Float;
use nalgebra::base::dimension::{U2, U3};
pub type Point<T, N> = nalgebra::Point<T, N>;
pub type Point2f = nalgebra::Point2<Float>;
pub type Point2i = nalgebra::Point2<Integer>;
pub type Point3f = nalgebra::Point3<Float>;
pub type Point3i = nalgebra::Point3<Integer>;
