use super::{Shape, ShapeIntersect, ShapePoint};
use crate::{
    def::Float,
    geometry::{Bounds3f, Point3f, Ray, Transform, Transformable},
    sampler::Sampler,
    Vector3f,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransformShape {
    shape: Arc<dyn Shape>,
    transform: Transform,
}

impl TransformShape {
    pub fn new(shape: Arc<dyn Shape>, transform: Transform) -> Self {
        Self {
            shape,
            transform,
        }
    }
    pub fn inverse_transform(&self) -> Transform {
        self.transform.clone().inverse()
    }
}

impl From<Arc<dyn Shape>> for TransformShape {
    fn from(shape: Arc<dyn Shape>) -> Self {
        Self {
            shape,
            transform: Transform::default(),
        }
    }
}

impl Transformable for TransformShape {
    fn apply(self, transform_: &Transform) -> Self {
        let transform = self.transform.apply(transform_);
        Self {
            shape: self.shape,
            transform,
        }
    }
}

impl Shape for TransformShape {
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.shape
            .intersect_predicate(&ray.apply(&self.inverse_transform()))
    }
    fn bound(&self) -> Bounds3f {
        self.shape.bound().apply(&self.transform)
    }
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect> {
        let intersect: Option<ShapeIntersect> =
            self.shape.intersect(&ray.apply(&self.inverse_transform()));
        Some(intersect?.apply(&self.transform))
    }
    fn by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        self.shape.by_point_pdf(
            &point.apply(&self.inverse_transform()),
            &shape_point.apply(&self.inverse_transform()),
        )
    }
    fn sample_by_point(&self, point: &Point3f, sampler: &mut dyn Sampler) -> (ShapePoint, Float) {
        let point = point.apply(&self.inverse_transform());
        let (shape_point, pdf) = self.shape.sample_by_point(&point, sampler);
        (shape_point.apply(&self.transform), pdf)
    }
    fn pdf(&self, shape_point: &ShapePoint) -> Float {
        self.shape.pdf(&shape_point.apply(&self.inverse_transform()))
    }
    fn sample(&self, sampler: &mut dyn Sampler) -> (ShapePoint, Float) {
        let (shape_point, pdf) = self.shape.sample(sampler);
        (shape_point.apply(&self.transform), pdf)
    }
    fn area(&self) -> Float {
        self.shape.area()
    }
    fn default_sample_by_point(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> (ShapePoint, Float) {
        let (shape_point, pdf) = self
            .shape
            .default_sample_by_point(&point.apply(&self.inverse_transform()), sampler);
        (shape_point.apply(&self.transform), pdf)
    }
    fn default_by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        self.shape.default_by_point_pdf(
            &point.apply(&self.inverse_transform()),
            &shape_point.apply(&self.inverse_transform()),
        )
    }
    fn by_point_w_pdf(&self, point: &Point3f, w: &Vector3f) -> Float {
        self.shape.by_point_w_pdf(
            &point.apply(&self.inverse_transform()),
            &w.apply(&self.inverse_transform()),
        )
    }
}
