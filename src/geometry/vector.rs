use super::{Transform, Transformable};
use crate::{
    def::{Float, Integer},
    scene_file_parser::{BasicTypes, ParseFromProperty},
};
use nalgebra::base::dimension::{U2, U3};
pub type Vector<T, D> = nalgebra::VectorN<T, D>;
pub type Vector2f = nalgebra::VectorN<Float, U2>;
pub type Vector2i = nalgebra::VectorN<Integer, U2>;
pub type Vector2u = nalgebra::VectorN<usize, U2>;
pub type Vector3f = nalgebra::VectorN<Float, U3>;
pub type Vector3i = nalgebra::VectorN<Integer, U3>;

impl Transformable for Vector3f {
    fn apply(self, transform: &Transform) -> Self {
        (transform.m * self.to_homogeneous()).xyz()
    }
}

impl ParseFromProperty for Vector3f {
    fn parse_from_property(_: &str, basic_type: &BasicTypes) -> Self {
        let floats = basic_type.get_floats().unwrap();
        Vector3f::new(floats[0], floats[1], floats[2])
    }
    fn parse_default() -> Self {
        Vector3f::new(0.,0.,0.)
    }
}
