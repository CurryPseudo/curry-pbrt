use crate::*;
use std::sync::Arc;
pub trait Fresnel {
    fn evaluate(&self, cos_i: Float) -> Spectrum;
}
pub struct SpecularReflection {
    r: Spectrum,
    fresnel: Arc<dyn Fresnel>,
}

impl SpecularReflection {
    pub fn new(r: Spectrum, fresnel: Arc<dyn Fresnel>) -> Self {
        Self { r, fresnel }
    }
}

impl DeltaBxDF for SpecularReflection {
    fn sample_f(&self, wo: &Vector3f) -> (Vector3f, Option<Spectrum>) {
        let wi = Vector3f::new(-wo.x, -wo.y, wo.z);
        let cos_theta_i = self.cos_theta(&wi);
        let s = self.r * self.fresnel.evaluate(cos_theta_i) / cos_theta_i.abs();
        (wi, Some(s))
    }
}

pub struct FresnelDielectric {}

impl FresnelDielectric {
    pub fn new(eta_i: Float, eta_t: Float) -> Self {
        Self {}
    }
}

impl Fresnel for FresnelDielectric {
    fn evaluate(&self, cos_i: Float) -> Spectrum {
        Spectrum::new(1.)
    }
}
