use crate::*;
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct GlassMaterial {
    r: Texture<Spectrum>,
    t: Texture<Spectrum>,
    eta: Texture<Float>,
}

impl GlassMaterial {
    pub fn new(r: Texture<Spectrum>, t: Texture<Spectrum>, eta: Texture<Float>) -> Self {
        Self { r, t, eta }
    }
}

impl Material for GlassMaterial {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> BSDF {
        let uv = shape_intersect.get_uv();
        let r = self.r.evaluate(uv);
        let t = self.t.evaluate(uv);
        let eta = self.eta.evaluate(uv);
        let mut bsdf = BSDF::new(*shape_intersect.get_normal(), *shape_intersect.get_normal());

        let fresnel = Box::new(FresnelDielectric::new(1., eta));
        bsdf.add_delta_bxdf(Arc::new(SpecularReflection::new(r, fresnel)));
        bsdf.add_delta_bxdf(Arc::new(SpecularTransmission::new(t, 1., eta)));
        bsdf
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
