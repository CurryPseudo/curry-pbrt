use crate::*;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct MirrorMaterial {
    r: Arc<dyn Texture<Spectrum>>,
}

impl MirrorMaterial {
    pub fn new(r: Arc<dyn Texture<Spectrum>>) -> Self {
        Self { r }
    }
}

impl Material for MirrorMaterial {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> BSDF {
        let r = self.r.evaluate(shape_intersect.get_uv());
        let mut bsdf = BSDF::new(*shape_intersect.get_normal(), *shape_intersect.get_normal());
        bsdf.add_delta_bxdf(Arc::new(SpecularReflection::new(
            r,
            Box::new(FresnelNoOp::default()),
        )));
        bsdf
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
