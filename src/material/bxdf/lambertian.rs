use super::BxDF;
use crate::*;

pub struct LambertianReflection {
    r: Spectrum,
}

impl LambertianReflection {
    pub fn new(r: Spectrum) -> Self {
        LambertianReflection { r }
    }
}

impl BxDF for LambertianReflection {
    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Option<Spectrum> {
        Some(self.r * INV_PI)
    }
}

pub struct LambertianTransmission {
    t: Spectrum,
}

impl LambertianTransmission {
    pub fn new(t: Spectrum) -> Self {
        Self { t }
    }
}

impl BxDF for LambertianTransmission {
    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Option<Spectrum> {
        Some(self.t * INV_PI)
    }
    fn bxdf_type(&self) -> BxDFType {
        BxDFType::Transmit
    }
}
