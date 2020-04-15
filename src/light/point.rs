use super::{DeltaLight, Light};
use crate::{
    def::Float,
    geometry::{Point3f, Ray, Transform, Transformable, Vector3f},
    math::WithPdf,
    sampler::SamplerWrapper,
    spectrum::Spectrum,
};

#[derive(Clone)]
pub struct PointLight {
    point: Point3f,
    i: Spectrum,
}

impl PointLight {
    pub fn new(i: Spectrum) -> Self {
        Self {
            point: Point3f::new(0., 0., 0.),
            i,
        }
    }
}

impl Transformable for PointLight {
    fn apply(self, transform: &Transform) -> Self {
        Self {
            point: self.point.apply(transform),
            i: self.i,
        }
    }
}

impl DeltaLight for PointLight {
    fn sample_li(&self, point: &Point3f) -> (Vector3f, Option<Spectrum>) {
        let wi = self.point - point;
        let i = self.i / wi.magnitude_squared();
        (wi.normalize(), Some(i))
    }
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray {
        let t = (self.point - point).magnitude();
        Ray::new(*point, *wi, t)
    }
}
