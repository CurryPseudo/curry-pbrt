use crate::*;
use std::sync::Arc;
#[derive(Debug)]
pub struct UberMaterial {
    pub kd: Arc<dyn Texture<Spectrum>>,
    pub ks: Arc<dyn Texture<Spectrum>>,
    pub kr: Arc<dyn Texture<Spectrum>>,
    pub kt: Arc<dyn Texture<Spectrum>>,
    pub roughness: Arc<dyn Texture<Float>>,
    pub uroughness: Option<Arc<dyn Texture<Float>>>,
    pub vroughness: Option<Arc<dyn Texture<Float>>>,
    pub eta: Arc<dyn Texture<Float>>,
    pub opacity: Arc<dyn Texture<Spectrum>>,
}
impl Material for UberMaterial {
    fn compute_scattering_functions(&self, intersect: &ShapeIntersect) -> BSDF {
        let mut bsdf = BSDF::new(*intersect.get_normal(), *intersect.get_normal());
        let uv = intersect.get_uv();
        let eta = self.eta.evaluate(uv);
        let opacity = self.opacity.evaluate(uv);
        let t = Spectrum::new(1.) - opacity;
        if !t.is_black() {
            bsdf.add_delta_bxdf(Arc::new(SpecularTransmission::new(t, 1., 1.)));
        }
        let kd = opacity * self.kd.evaluate(uv);
        if !kd.is_black() {
            bsdf.add_bxdf(Arc::new(Lambertian::new(kd)));
        }

        let ks = opacity * self.ks.evaluate(uv);
        if !ks.is_black() {
            let fresnel = FresnelDielectric::new(1., eta);
            let mut roughu = self
                .uroughness
                .as_ref()
                .map(|uroughness| uroughness.evaluate(uv))
                .unwrap_or_else(|| self.roughness.evaluate(uv));
            let mut roughv = self
                .vroughness
                .as_ref()
                .map(|vroughness| vroughness.evaluate(uv))
                .unwrap_or(roughu);
            roughu = TrowbridgeReitzDistribution::roughness_to_alpha(roughu);
            roughv = TrowbridgeReitzDistribution::roughness_to_alpha(roughv);
            let distribution = TrowbridgeReitzDistribution::new(roughu, roughv);
            bsdf.add_bxdf(Arc::new(MicrofacetReflection::new(
                ks,
                distribution,
                fresnel,
            )));
        }
        let kr = opacity * self.kr.evaluate(uv);
        if !kr.is_black() {
            let fresnel = FresnelDielectric::new(1., eta);
            bsdf.add_delta_bxdf(Arc::new(SpecularReflection::new(kr, Box::new(fresnel))));
        }
        let kt = opacity * self.kt.evaluate(uv);
        if !kt.is_black() {
            bsdf.add_delta_bxdf(Arc::new(SpecularTransmission::new(kt, 1., eta)));
        }
        bsdf
    }
    fn box_clone(&self) -> Box<(dyn Material + 'static)> {
        todo!()
    }
}
