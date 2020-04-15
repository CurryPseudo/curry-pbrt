use super::Light;
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

impl Light for PointLight {
    fn le(&self, _: &Ray) -> Option<Spectrum> {
        None
    }
    fn sample_li(
        &self,
        point: &Point3f,
        sampler: &mut SamplerWrapper,
    ) -> WithPdf<(Vector3f, Option<Spectrum>)> {
        sampler.get_2d();
        let wi = self.point - point;
        let i = self.i / wi.magnitude_squared();
        WithPdf::new((wi.normalize(), Some(i)), 1.)
    }
    fn pdf(&self, _: &Ray) -> Float {
        0.
    }
    fn box_apply(&mut self, transform: &Transform) -> Box<dyn Light> {
        Box::new(self.clone().apply(transform))
    }
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray {
        let t = (self.point - point).magnitude();
        Ray::new(*point, *wi, t)
    }
}
