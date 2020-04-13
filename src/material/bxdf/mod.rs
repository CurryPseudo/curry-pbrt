mod lambertian_reflection;
pub use lambertian_reflection::*;
use crate::{geometry::{cosine_sample_hemisphere, Vector3f}, sampler::SamplerWrapper, spectrum::Spectrum, math::{INV_PI, WithPdf}, def::Float};

pub trait BRDF {
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum>;
    fn sample_f(&self, wo: &Vector3f, sampler: &mut SamplerWrapper) -> WithPdf<(Vector3f, Option<Spectrum>)> {
        let wi = cosine_sample_hemisphere(sampler.get_2d());
        let f = self.f(wo, &wi);
        WithPdf::new((wi.t, f), wi.pdf)
    }
    fn pdf(&self, _wo: &Vector3f, wi: &Vector3f) -> Float {
        wi.z * INV_PI
    }
    fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f) -> WithPdf<Option<Spectrum>> {
        WithPdf::new(self.f(wo, wi), self.pdf(wo, wi))
    }
}

