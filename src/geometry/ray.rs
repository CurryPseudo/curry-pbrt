use super::{Transform, Transformable};
use crate::def::Float;
use crate::geometry::point::Point3f;
use crate::geometry::vector::Vector3f;

#[derive(Debug, Clone)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_max: Float,
}

impl Ray {
    pub fn new(o: Point3f, d: Vector3f, t_max: Float) -> Self {
        Self { o, d, t_max }
    }
    pub fn eval(&self, t: Float) -> Point3f {
        &self.o + &self.d * t
    }
}

impl Transformable for Ray {
    fn apply(mut self, transform: &Transform) -> Self {
        self.o = self.o.apply(transform);
        self.d = self.d.apply(transform);
        self
    }
}
pub struct RayDiffrential {
    r: Ray,
    rx: Ray,
    ry: Ray,
}
