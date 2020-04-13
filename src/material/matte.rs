use super::{bxdf::BRDF, Material};
use crate::{geometry::ShapeIntersect, spectrum::Spectrum, texture::Texture};

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
        let kd = self.kd.evaluate(&shape_intersect.uv);
        Box::new(super::bxdf::LambertianReflection::new(kd))
    }
}