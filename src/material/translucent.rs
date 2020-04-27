use crate::*;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct TranslucentMaterial {
    pub kd: Arc<dyn Texture<Spectrum>>,
    pub ks: Arc<dyn Texture<Spectrum>>,
    pub roughness: Arc<dyn Texture<Float>>,
    pub reflect: Arc<dyn Texture<Spectrum>>,
    pub transmit: Arc<dyn Texture<Spectrum>>,
}

impl Material for TranslucentMaterial {
    fn compute_scattering_functions(&self, intersect: &ShapeIntersect) -> BSDF {
        let eta = 1.5;
        let mut bsdf = BSDF::new(*intersect.get_normal(), *intersect.get_normal());
        let uv = intersect.get_uv();
        let r = self.reflect.evaluate(uv);
        let t = self.transmit.evaluate(uv);
        if r.is_black() && t.is_black() {
            return bsdf;
        }

        let kd = self.kd.evaluate(uv);
        if !kd.is_black() {
            if !r.is_black() {
                bsdf.add_bxdf(Arc::new(LambertianReflection::new(r * kd)));
            }
            if !t.is_black() {
                bsdf.add_bxdf(Arc::new(LambertianTransmission::new(t * kd)));
            }
        }

        let ks = self.ks.evaluate(uv);
        if !ks.is_black() {
            let mut rough = self.roughness.evaluate(uv);
            rough = TrowbridgeReitzDistribution::roughness_to_alpha(rough);
            let distribution = TrowbridgeReitzDistribution::new(rough, rough);
            if !r.is_black() {
                let fresnel = FresnelDielectric::new(1., eta);
                bsdf.add_bxdf(Arc::new(MicrofacetReflection::new(
                    r * ks,
                    distribution,
                    fresnel,
                )));
            }

            if !t.is_black() {
                bsdf.add_bxdf(Arc::new(MicrofacetTransmission::new(
                    t * ks,
                    distribution,
                    1.,
                    eta,
                )));
            }
        }
        bsdf
    }
    fn box_clone(&self) -> Box<(dyn Material + 'static)> {
        Box::new(self.clone())
    }
}
