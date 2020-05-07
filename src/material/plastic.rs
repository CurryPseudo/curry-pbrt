use crate::*;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct PlasticMaterial {
    pub kd: Arc<dyn Texture<Spectrum>>,
    pub ks: Arc<dyn Texture<Spectrum>>,
    pub roughness: Arc<dyn Texture<Float>>,
}
impl Material for PlasticMaterial {
    fn compute_scattering_functions(&self, intersect: &ShapeIntersect) -> BSDF {
        let mut bsdf = BSDF::new(*intersect.get_normal(), *intersect.get_normal());
        let uv = intersect.get_uv();
        let kd = self.kd.evaluate(uv);
        if !kd.is_black() {
            bsdf.add_bxdf(Arc::new(LambertianReflection::new(kd)));
        }

        let ks = self.ks.evaluate(uv);
        if !ks.is_black() {
            let fresnel = FresnelDielectric::new(1., 1.5);
            let mut rough = self.roughness.evaluate(uv);
            rough = TrowbridgeReitzDistribution::roughness_to_alpha(rough);
            let distribution = TrowbridgeReitzDistribution::new(rough, rough);
            bsdf.add_bxdf(Arc::new(MicrofacetReflection::new(
                ks,
                distribution,
                fresnel,
            )));
        }
        bsdf
    }
    fn box_clone(&self) -> Box<(dyn Material + 'static)> {
        Box::new(self.clone())
    }
}
