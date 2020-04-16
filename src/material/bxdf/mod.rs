mod lambertian_reflection;
pub use lambertian_reflection::*;
use crate::{geometry::{cosine_sample_hemisphere, Vector3f, Normal3f}, sampler::Sampler, spectrum::Spectrum, math::{INV_PI, WithPdf}, def::Float};

pub trait BRDF {
    fn brdf_f(&self, wo: &Vector3f, wi: &Vector3f) -> Option<Spectrum>;
    fn f(&self, wo: &Vector3f, wi: &Vector3f, n: &Normal3f) -> Option<Spectrum> {
        if wo.dot(n) * wi.dot(n) <= 0. {
            None
        }
        else {
            self.brdf_f(wo, wi)
        }
    }
    fn sample_f(&self, wo: &Vector3f, sampler: &mut dyn Sampler) -> WithPdf<(Vector3f, Option<Spectrum>)> {
        let wi = cosine_sample_hemisphere(sampler.get_2d());
        let f = self.brdf_f(wo, &wi);
        WithPdf::new((wi.t, f), wi.pdf)
    }
    fn pdf(&self, _wo: &Vector3f, wi: &Vector3f) -> Float {
        wi.z * INV_PI
    }
    fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f, n: &Normal3f) -> WithPdf<Option<Spectrum>> {
        WithPdf::new(self.f(wo, wi, n), self.pdf(wo, wi))
    }
}

