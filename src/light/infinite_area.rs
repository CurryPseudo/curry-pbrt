use crate::*;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct InfiniteAreaLight {
    map: Arc<dyn Texture<Spectrum>>,
    map_distribution: Distribution2D
}

impl InfiniteAreaLight {
    pub fn new(map: Arc<dyn Texture<Spectrum>>) -> Self {
        let pixels = map.pixels();
        let map_distribution = {
            let size = pixels.size();
            let mut f = Vec::new();
            for (i, s) in pixels.enumerate() {
                let theta = (i.y as Float + 0.5) / size.y as Float;
                f.push(s.y() * theta.sin());
            }
            let f_2d = FixedVec2D::from_vec(f, size.x);
            Distribution2D::from(f_2d)
        };
        Self {
            map, map_distribution
        }
    }
}

impl Light for InfiniteAreaLight {
    fn box_apply(&self, _: &Transform) -> Box<dyn Light> {
        Box::new(self.clone())
    }
    fn is_delta(&self) -> bool {
        false
    }
    fn le_out_scene(&self, ray: &Ray) -> Option<Spectrum> {
        let mut uv = spherical_to_normalize_phi_theta(&ray.d.normalize());
        uv.y = 1. - uv.y;
        Some(self.map.evaluate(&uv))
    }
    fn out_scene_pdf(&self, ray: &Ray) -> Float {
        let mut uv = spherical_to_normalize_phi_theta(&ray.d.normalize());
        uv.y = 1. - uv.y;
        let mut pdf = self.map_distribution.continuous_pdf(uv);
        let sin_theta = (uv.y * PI).sin();
        pdf = if sin_theta != 0. {
            pdf / (2. * PI * PI * sin_theta)
        }
        else {
            0.
        };
        pdf
    }
    fn sample_li(
        &self,
        point: &ShapePoint,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float, VisibilityTester) {
        let u = sampler.get_2d();
        let (_, mut pdf, mut uv) = self.map_distribution.sample_continuous(u);
        uv.y = 1. - uv.y;
        let wi = normalize_phi_theta_to_spherical(&uv);
        let sin_theta = (uv.y * PI).sin();
        pdf = if sin_theta != 0. {
            pdf / (2. * PI * PI * sin_theta)
        }
        else {
            0.
        };
        let le = self.map.evaluate(&uv);
        let visibility_tester = VisibilityTester::new_od(point, &wi);
        (wi, Some(le), pdf, visibility_tester)
    }
}
