use crate::{math::WithPdf, geometry::{Ray, Vector3f, Point3f}, sampler::SamplerWrapper, spectrum::Spectrum, def::Float};

mod point;

pub use point::*;

pub trait Light {
    fn sample_li(&self, point: Point3f, sampler: &mut SamplerWrapper) -> WithPdf<(Vector3f, Option<Spectrum>)>;
    fn le(&self, ray: &Ray) -> Option<Spectrum>;
    fn pdf(&self, ray: &Ray) -> Float;
    fn le_pdf(&self, ray: &Ray) -> WithPdf<Option<Spectrum>> {
        WithPdf::new(self.le(ray), self.pdf(ray))
    }
}

