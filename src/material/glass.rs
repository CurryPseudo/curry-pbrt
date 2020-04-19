use crate::*;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct GlassMaterial {
    r: Texture<Spectrum>,
}

impl GlassMaterial {
    pub fn new(r: Texture<Spectrum>) -> Self {
        Self { r }
    }
}

impl Material for GlassMaterial {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> BSDF {
        let r = self.r.evaluate(shape_intersect.get_uv());
        let mut bsdf = BSDF::new(*shape_intersect.get_normal(), *shape_intersect.get_normal());
        let fresnel = Arc::new(FresnelDielectric::new(1., 1.5));
        bsdf.add_delta_bxdf(Arc::new(SpecularReflection::new(r, fresnel)));
        bsdf
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
