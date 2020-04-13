use super::Shape;
use crate::geometry::{Bounds3f, Ray, RayIntersectCache, Transform, Transformable};

pub struct TransformShape<T> {
    t: T,
    transform: Transform,
    inverse_transform: Transform,
}

impl<T> From<T> for TransformShape<T> {
    fn from(t: T) -> Self {
        Self {
            t,
            transform: Transform::default(),
            inverse_transform: Transform::default(),
        }
    }
}

impl<T> Transformable for TransformShape<T> {
    fn apply(self, transform: &Transform) -> Self {
        let inverse_transform = self.transform.clone().inverse();
        Self {
            t: self.t,
            transform: self.transform.apply(transform),
            inverse_transform,
        }
    }
}

impl<T: Shape> Shape for TransformShape<T> {
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.t
            .intersect_predicate(&ray.apply(&self.inverse_transform))
    }
    fn bound(&self) -> Bounds3f {
        self.t.bound().apply(&self.transform)
    }
    fn intersect(&self, ray: &Ray) -> Option<super::ShapeIntersect> {
        self.t.intersect(&ray.apply(&self.inverse_transform))
    }
}
