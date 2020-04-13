use super::{Bounds3f, Normal3f, Ray, RayIntersectCache, Point3f, Point2f};
use crate::def::Float;
mod sphere;
mod transform;
pub use sphere::*;
pub use transform::*;

pub trait Shape {
    fn bound(&self) -> Bounds3f;
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect>;
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
    fn intersect_predicate_through_bound(&self, ray: &RayIntersectCache) -> bool {
        if self.bound().intersect_predicate_cached(ray) {
            self.intersect_predicate(ray.origin_ray())
        } else {
            false
        }
    }
}

pub struct ShapeIntersect {
    pub p: Point3f,
    pub n: Normal3f,
    pub t: Float,
    pub uv: Point2f,
}

impl ShapeIntersect {
    pub fn new(p: Point3f, n: Normal3f, t: Float, uv: Point2f) -> Self { Self { p, n, t, uv } }
}


