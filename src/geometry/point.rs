use super::{Transform, Transformable, Vector2f, Vector3f};
use crate::def::Float;
use crate::{def::Integer, math::PI};
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
pub fn concentric_sample_disk(u: Point2f) -> Point2f {
    let u = 2. * u - Vector2f::new(1., 1.);
    if u.x == 0. || u.y == 0. {
        Point2f::new(0., 0.)
    } else {
        let (r, theta) = if u.x.abs() > u.y.abs() {
            (u.x, (PI / 4.) * (u.y / u.x))
        } else {
            (u.y, (PI / 2.) - (PI / 4.) * (u.x / u.y))
        };
        polar_point(r, theta)
    }
}
pub fn cosine_sample_hemisphere(u: Point2f) -> Vector3f {
    let d = concentric_sample_disk(u);
    let z = (1. - d.coords.magnitude_squared()).sqrt();
    Vector3f::new(d.x, d.y, z)
}
