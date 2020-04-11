mod lambertian_reflection;
use crate::{geometry::{cosine_sample_hemisphere, Vector3f}, sampler::SamplerWrapper, spectrum::Spectrum, math::WithPdf};

pub trait BRDF {
    fn f(&self, wo: Vector3f, wi: Vector3f) -> Spectrum;
    fn sample_f(&self, wo: Vector3f, sampler: &mut SamplerWrapper) -> WithPdf<Spectrum> {
        let wi = cosine_sample_hemisphere(sampler.get_2d());
        wi.map(|wi| self.f(wi, wo))
    }
}

