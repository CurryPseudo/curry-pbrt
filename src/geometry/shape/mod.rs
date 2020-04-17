use super::{
    Bounds3f, Normal3f, Point2f, Point3f, Ray, RayIntersectCache, Transform, Transformable,
};
use crate::{def::Float, sampler::Sampler, scene_file_parser::PropertySet};
mod sphere;
mod transform;
use downcast_rs::DowncastSync;
pub use sphere::*;
pub use transform::*;

pub trait Shape: DowncastSync {
    fn area(&self) -> Float;
    fn bound(&self) -> Bounds3f;
    fn sample(&self, sampler: &mut dyn Sampler) -> ShapePoint;
    fn pdf(&self, _: &ShapePoint) -> Float {
        1. / self.area()
    }
    fn sample_pdf(&self, sampler: &mut dyn Sampler) -> (ShapePoint, Float) {
        let shape_point = self.sample(sampler);
        (shape_point, self.pdf(&shape_point))
    }
    fn sample_by_point(&self, point: &Point3f, sampler: &mut dyn Sampler) -> ShapePoint;
    fn default_by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        let d = point - shape_point.p;
        let distance_2 = d.magnitude_squared();
        let distance = distance_2.sqrt();
        distance_2 / ((d / distance).dot(&shape_point.n).abs() * self.area())
    }
    fn by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        self.default_by_point_pdf(point, shape_point)
    }
    fn sample_by_point_pdf(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> (ShapePoint, Float) {
        let shape_point = self.sample_by_point(point, sampler);
        let pdf = self.by_point_pdf(point, &shape_point);
        (shape_point, pdf)
    }
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect>;
    fn intersect_through_bound(&self, ray: &RayIntersectCache) -> Option<ShapeIntersect> {
        if self.bound().intersect_predicate_cached(ray) {
            self.intersect(ray.origin_ray())
        } else {
            None
        }
    }
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
    fn box_clone(&self) -> Box<dyn Shape>;
}

impl_downcast!(sync Shape);

pub fn shape_apply(shape: Box<dyn Shape>, transform: &Transform) -> Box<dyn Shape> {
    match shape.downcast::<TransformShape>() {
        Ok(transfrom_shape) => Box::new(transfrom_shape.apply(transform)),
        Err(shape) => Box::new(TransformShape::from(shape).apply(transform)),
    }
}

#[derive(Clone, Copy)]
pub struct ShapePoint {
    pub p: Point3f,
    pub n: Normal3f,
    pub uv: Point2f,
}

impl ShapePoint {
    pub fn new(p: Point3f, n: Normal3f, uv: Point2f) -> Self {
        Self { p, n, uv }
    }
}

#[derive(Clone, Copy)]
pub struct ShapeIntersect {
    t: Float,
    p: ShapePoint,
}

impl Transformable for ShapePoint {
    fn apply(self, transform: &super::Transform) -> Self {
        let p = self.p.apply(transform);
        let n = self.n.apply(transform);
        Self { p, n, uv: self.uv }
    }
}
impl ShapeIntersect {
    pub fn new(p: Point3f, n: Normal3f, t: Float, uv: Point2f) -> Self {
        Self {
            t,
            p: ShapePoint::new(p, n, uv),
        }
    }
    pub fn get_point(&self) -> &Point3f {
        &self.p.p
    }
    pub fn get_normal(&self) -> &Normal3f {
        &self.p.n
    }
    pub fn get_uv(&self) -> &Point2f {
        &self.p.uv
    }
    pub fn get_shape_point(&self) -> &ShapePoint {
        &self.p
    }

    pub fn get_t(&self) -> Float {
        self.t
    }
}

impl Transformable for ShapeIntersect {
    fn apply(self, transform: &Transform) -> Self {
        Self {
            t: self.t,
            p: self.p.apply(transform),
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
