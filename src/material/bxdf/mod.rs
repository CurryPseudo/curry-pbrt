mod lambertian;
mod oren_nayar;
pub use lambertian::*;
pub use oren_nayar::*;
use crate::*;

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
    fn sample_f(&self, wo: &Vector3f, sampler: &mut dyn Sampler) -> (Vector3f, Option<Spectrum>, Float) {
        let (wi, pdf) = cosine_sample_hemisphere(sampler.get_2d());
        let f = self.brdf_f(wo, &wi);
        (wi, f, pdf)
    }
    fn pdf(&self, _wo: &Vector3f, wi: &Vector3f) -> Float {
        wi.z * INV_PI
    }
    fn f_pdf(&self, wo: &Vector3f, wi: &Vector3f, n: &Normal3f) -> (Option<Spectrum>, Float) {
        (self.f(wo, wi, n), self.pdf(wo, wi))
    }
}

