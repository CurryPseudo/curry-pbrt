use super::BRDF;
use crate::{geometry::Vector3f, spectrum::Spectrum, math::INV_PI};

pub struct Lambertian {
    r: Spectrum,
}

impl Lambertian {
    pub fn new(r: Spectrum) -> Self { Self { r } }
}


impl BRDF for Lambertian {
    fn brdf_f(&self, _: &Vector3f, _: &Vector3f) -> Option<Spectrum> {
        Some(self.r.clone() / INV_PI)
    }
}
