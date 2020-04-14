use super::{Shape, ShapeIntersect};
use crate::geometry::{Bounds3f, Ray, Transform, Transformable};

pub struct TransformShape {
    shape: Box<dyn Shape>,
    transform: Transform,
    inverse_transform: Transform,
}

impl From<Box<dyn Shape>> for TransformShape {
    fn from(shape: Box<dyn Shape>) -> Self {
        Self {
            shape,
            transform: Transform::default(),
            inverse_transform: Transform::default(),
        }
    }
}

impl Transformable for TransformShape {
    fn apply(self, transform: &Transform) -> Self {
        let transform = self.transform.apply(transform);
        let inverse_transform = transform.clone().inverse();
        Self {
            shape: self.shape,
            transform,
            inverse_transform,
        }
    }
}

impl Shape for TransformShape {
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.shape
            .intersect_predicate(&ray.apply(&self.inverse_transform))
    }
    fn bound(&self) -> Bounds3f {
        self.shape.bound().apply(&self.transform)
    }
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect> {
        let intersect = self.shape.intersect(&ray.apply(&self.inverse_transform));
        Some(intersect?.apply(&self.transform))
    }
}
