use super::BxDF;
use crate::*;

pub struct Lambertian {
    r: Spectrum,
}

impl Lambertian {
    pub fn new(r: Spectrum) -> Self { Self { r } }
}


impl BxDF for Lambertian {
    fn f(&self, _: &Vector3f, _: &Vector3f) -> Option<Spectrum> {
        Some(self.r.clone() * INV_PI)
    }
}
