use super::{Shape, ShapeIntersect, ShapePoint};
use crate::{
    def::Float,
    geometry::{Bounds3f, Point3f, Ray, Transform, Transformable},
    sampler::Sampler,
};

pub struct TransformShape {
    shape: Box<dyn Shape>,
    transform: Transform,
    inverse_transform: Transform,
    bound: Bounds3f,
}

impl From<Box<dyn Shape>> for TransformShape {
    fn from(shape: Box<dyn Shape>) -> Self {
        let bound = shape.bound();
        Self {
            shape,
            transform: Transform::default(),
            inverse_transform: Transform::default(),
            bound,
        }
    }
}

impl Transformable for TransformShape {
    fn apply(self, transform: &Transform) -> Self {
        let transform = self.transform.apply(transform);
        let bound = self.bound.apply(&transform);
        let inverse_transform = transform.clone().inverse();
        Self {
            shape: self.shape,
            transform,
            inverse_transform,
            bound,
        }
    }
}

impl Shape for TransformShape {
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.shape
            .intersect_predicate(&ray.apply(&self.inverse_transform))
    }
    fn bound(&self) -> Bounds3f {
        self.bound.clone()
    }
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect> {
        let intersect: Option<ShapeIntersect> =
            self.shape.intersect(&ray.apply(&self.inverse_transform));
        Some(intersect?.apply(&self.transform))
    }
    fn sample_by_point(&self, point: &Point3f, sampler: &mut dyn Sampler) -> ShapePoint {
        self.shape
            .sample_by_point(&point.apply(&self.inverse_transform), sampler)
            .apply(&self.transform)
    }
    fn by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        self.shape.by_point_pdf(
            &point.apply(&self.inverse_transform),
            &shape_point.apply(&self.inverse_transform).clone(),
        )
    }
    fn sample(&self, sampler: &mut dyn Sampler) -> ShapePoint {
        self.shape.sample(sampler).apply(&self.transform)
    }
    fn sample_by_point_pdf(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> (ShapePoint, Float) {
        let point = point.apply(&self.inverse_transform);
        let shape_point = self.shape.sample_by_point(&point, sampler);
        let pdf = self.shape.by_point_pdf(&point, &shape_point);
        (shape_point.apply(&self.transform), pdf)
    }
    fn pdf(&self, shape_point: &ShapePoint) -> Float {
        self.shape.pdf(&shape_point.apply(&self.inverse_transform))
    }
    fn sample_pdf(&self, sampler: &mut dyn Sampler) -> (ShapePoint, Float) {
        let shape_point = self.shape.sample(sampler);
        (
            shape_point.apply(&self.transform),
            self.shape.pdf(&shape_point),
        )
    }
    fn area(&self) -> Float {
        self.shape.area()
    }
    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(Self {
            shape: self.shape.box_clone(),
            transform: self.transform.clone(),
            inverse_transform: self.inverse_transform.clone(),
            bound: self.bound.clone(),
        })
    }
}
