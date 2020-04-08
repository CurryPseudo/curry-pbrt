use super::BxDF;
use crate::{def::Float, geometry::Vector3f, spectrum::Spectrum, sampler::SamplerWrapper, math::INV_PI};

pub struct LambertianReflection {
    r: Spectrum,
}

impl BxDF for LambertianReflection {
    fn f(&self, _: Vector3f, _: Vector3f) -> Spectrum {
        self.r.clone() / INV_PI
    }
}
