use crate::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MatteMaterial {
    kd: Arc<dyn Texture<Spectrum>>,
    sigma: Arc<dyn Texture<Float>>,
}

impl MatteMaterial {
    pub fn new(kd: Arc<dyn Texture<Spectrum>>, sigma: Arc<dyn Texture<Float>>) -> Self {
        Self { kd, sigma }
    }
}

impl Material for MatteMaterial {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> BSDF {
        let kd = self.kd.evaluate(shape_intersect.get_uv());
        let sigma = clamp(self.sigma.evaluate(shape_intersect.get_uv()), 0., 90.);
        let mut bsdf = BSDF::new(*shape_intersect.get_normal(), *shape_intersect.get_normal());
        if sigma == 0. {
            bsdf.add_bxdf(Arc::new(LambertianReflection::new(kd)));
        }
        else {
            bsdf.add_bxdf(Arc::new(OrenNayar::new(kd, sigma)))
        }
        bsdf
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
