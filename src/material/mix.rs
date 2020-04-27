use std::sync::Arc;
use crate::*;
#[derive(Debug, Clone)]
pub struct MixMaterial {
    pub m1: Arc<dyn Material>,
    pub m2: Arc<dyn Material>,
    pub scale: Arc<dyn Texture<Spectrum>>
}

impl Material for MixMaterial {
    fn compute_scattering_functions(&self, intersect: &ShapeIntersect) -> BSDF {
        let uv = intersect.get_uv();
        let scale = self.scale.evaluate(uv);
        let bsdf = self.m1.compute_scattering_functions(intersect);
        bsdf.mix(self.m2.compute_scattering_functions(intersect), scale)
    }
    fn box_clone(&self) -> Box<(dyn Material + 'static)> {
        Box::new(self.clone())
    }
}
