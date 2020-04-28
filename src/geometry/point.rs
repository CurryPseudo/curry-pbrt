use super::{Transform, Transformable};
use crate::*;
pub type Point<T, N> = nalgebra::Point<T, N>;
pub type Point2f = nalgebra::Point2<Float>;
pub type Point2i = nalgebra::Point2<Integer>;
pub type Point2u = nalgebra::Point2<usize>;
pub type Point3f = nalgebra::Point3<Float>;
pub type Point3i = nalgebra::Point3<Integer>;

impl Transformable for Point3f {
    fn apply(self, transform: &Transform) -> Self {
        Self::from_homogeneous(transform.m.as_ref() * self.to_homogeneous()).unwrap()
    }
}
impl ParseFromProperty for Point3f {
    fn parse_from_property(_: &str, basic_type: &BasicTypes) -> Self {
        let floats = basic_type.get_floats().unwrap();
        Point3f::new(floats[0], floats[1], floats[2])
    }
    fn parse_default() -> Self {
        Point3f::new(0., 0., 0.)
    }
}

impl ParseConsumeProperty for Point3f {
    fn consume_size() -> usize {
        3
    }
}
