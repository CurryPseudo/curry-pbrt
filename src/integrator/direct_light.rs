use super::Integrator;
use crate::*;

pub struct DirectLightIntegrator {
    max_depth: usize,
}

impl DirectLightIntegrator {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }
    fn li_depth(
        &self,
        ray: &Ray,
        scene: &Scene,
        sampler: &mut dyn Sampler,
        depth: usize,
    ) -> Spectrum {
        let mut l = Spectrum::new(0.);
        if let Some(intersect) = scene.intersect(ray) {
            if let Some(bsdf) = intersect.compute_scattering_functions() {
                let wo = -ray.d.normalize();
                let shape_point = &intersect.get_shape_intersect().get_shape_point();
                let n = &shape_point.n;
                l += uniform_sample_one_light(shape_point, &bsdf, &wo, scene, sampler);
                if depth + 1 < self.max_depth {
                    for (wi, s) in bsdf.sample_all_delta_f(&wo) {
                        let ray = Ray::new_shape_point_d(&shape_point, wi);
                        l += self.li_depth(&ray, scene, sampler, depth + 1) * s * n.dot(&wi).abs();
                    }
                }
            } else {
                l += intersect.le();
            }
        } else {
            for light in scene.get_lights() {
                l += light.le_out_scene(ray);
            }
        }
        l
    }
}

impl Integrator for DirectLightIntegrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Spectrum {
        self.li_depth(ray, scene, sampler, 0)
    }
}
