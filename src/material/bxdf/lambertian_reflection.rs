use super::BRDF;
use crate::{geometry::Vector3f, spectrum::Spectrum, math::INV_PI};

pub struct LambertianReflection {
    r: Spectrum,
}

impl BRDF for LambertianReflection {
    fn f(&self, _: Vector3f, _: Vector3f) -> Spectrum {
        self.r.clone() / INV_PI
    }
}
