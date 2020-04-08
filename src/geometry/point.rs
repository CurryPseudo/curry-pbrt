use super::{Transform, Transformable, Vector2f};
use crate::def::Float;
use crate::{math::PI, def::Integer};
use nalgebra::base::dimension::{U2, U3};
pub type Point<T, N> = nalgebra::Point<T, N>;
pub type Point2f = nalgebra::Point2<Float>;
pub type Point2i = nalgebra::Point2<Integer>;
pub type Point3f = nalgebra::Point3<Float>;
pub type Point3i = nalgebra::Point3<Integer>;

impl Transformable for Point3f {
    fn apply(self, transform: &Transform) -> Self {
        Self::from_homogeneous(transform.m * self.to_homogeneous()).unwrap()
    }
}
pub fn polar_point(r: Float, theta: Float) -> Point2f {
    r * Point2f::new(theta.cos(), theta.sin())
}
pub fn concentric_sample_disk(p: Point2f) -> Point2f {
    let p = 2. * p - Vector2f::new(1., 1.);
    if p.x == 0. || p.y == 0. {
        Point2f::new(0., 0.)
    } else {
        let (r, theta) = if p.x.abs() > p.y.abs() {
            (p.x, (PI / 4.) * (p.y / p.x))  
        } else {
            (p.y, (PI / 2.) - (PI / 4.) * (p.x / p.y))  
        };
        polar_point(r, theta)
    }
}
