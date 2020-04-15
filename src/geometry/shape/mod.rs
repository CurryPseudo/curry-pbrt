use super::{
    Bounds3f, Normal3f, Point2f, Point3f, Ray, RayIntersectCache, Transform, Transformable,
};
use crate::{def::Float, scene_file_parser::PropertySet};
mod sphere;
mod transform;
use downcast_rs::{DowncastSync, Downcast};
pub use sphere::*;
pub use transform::*;

pub trait Shape: DowncastSync {
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

impl_downcast!(sync Shape);

pub fn shape_apply(shape: Box<dyn Shape>, transform: &Transform) -> Box<dyn Shape> {
    match shape.downcast::<TransformShape>() {
        Ok(transfrom_shape) => Box::new(transfrom_shape.apply(transform)),
        Err(shape) => Box::new(TransformShape::from(shape).apply(transform)),
    }
}

#[derive(Debug)]
pub struct ShapeIntersect {
    pub p: Point3f,
    pub n: Normal3f,
    pub t: Float,
    pub uv: Point2f,
}

impl ShapeIntersect {
    pub fn new(p: Point3f, n: Normal3f, t: Float, uv: Point2f) -> Self {
        Self { p, n, t, uv }
    }
}

impl Transformable for ShapeIntersect {
    fn apply(self, transform: &super::Transform) -> Self {
        let p = self.p.apply(transform);
        let n = self.n.apply(transform);
        Self {
            p,
            n,
            t: self.t,
            uv: self.uv,
        }
    }
}
pub fn parse_shape(property_set: &PropertySet) -> Box<dyn Shape> {
    if property_set.get_name().unwrap() == "sphere" {
        let radius = property_set.get_value("radius").unwrap_or(1.);
        return Box::new(Sphere::new(radius));
    }
    panic!()
}
