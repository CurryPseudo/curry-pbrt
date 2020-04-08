mod lambertian_reflection;
use crate::{def::Float, geometry::Vector3f, sampler::SamplerWrapper, spectrum::Spectrum};

pub trait BxDF {
    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum;
    fn sample_f(&self, wo: Vector3f, sampler: &mut SamplerWrapper) -> BxDFSampleResult {
        unimplemented!()       
    }
}
pub struct BxDFSampleResult {
    pub r: Spectrum,
    pub pdf: Float,
}

impl BxDFSampleResult {
    pub fn new(r: Spectrum, pdf: Float) -> Self {
        Self { r, pdf }
    }
}
