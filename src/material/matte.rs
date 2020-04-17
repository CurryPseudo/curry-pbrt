use super::{bxdf::BRDF, Material};
use crate::*;

#[derive(Debug, Clone)]
pub struct MatteMaterial {
    kd: Texture<Spectrum>,
}

impl MatteMaterial {
    pub fn new(kd: Texture<Spectrum>) -> Self {
        Self { kd }
    }
}

impl Material for MatteMaterial {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> Box<dyn BRDF> {
        let kd = self.kd.evaluate(shape_intersect.get_uv());
        Box::new(super::bxdf::Lambertian::new(kd))
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
