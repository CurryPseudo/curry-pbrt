use super::Integrator;
use crate::{
    def::Float, geometry::Ray, math::power_heuristic, sampler::Sampler, scene::Scene,
    spectrum::Spectrum,
};

pub struct DirectLightIntegrator {}

impl DirectLightIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for DirectLightIntegrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Spectrum {
        let mut l = Spectrum::new(0.);
        if let Some(intersect) = scene.intersect(ray) {
            if let Some(brdf) = intersect.compute_scattering_functions() {
                let lights = scene.get_lights();
                if lights.len() > 0 {
                    let wo = -ray.d.normalize();
                    let light = &lights[(sampler.get_usize(lights.len())) as usize];
                    let point = &intersect.get_shape_intersect().get_point().clone();
                    let n = &intersect.get_shape_intersect().get_normal();
                    {
                        // sample light
                        if let (wi, Some(li), li_pdf) = light.sample_li(&point, sampler, scene) {
                            if let (Some(f), f_pdf) = brdf.f_pdf(&wo, &wi, n) {
                                if light.is_delta() {
                                    l += li * f * n.dot(&wi).abs() / li_pdf;
                                } else {
                                    l += li * f * n.dot(&wi).abs() * power_heuristic(li_pdf, f_pdf)
                                        / li_pdf;
                                }
                            }
                        }
                    }
                    if !light.is_delta() {
                        // sample brdf
                        if let (wi, Some(f), f_pdf) = brdf.sample_f(&wo, sampler) {
                            let mut ray = Ray::new_od(point.clone(), wi.clone());
                            ray.move_a_bit();
                            if let (Some(li), li_pdf) = light.le_pdf(&ray, scene) {
                                l += li * f * n.dot(&wi).abs() * power_heuristic(f_pdf, li_pdf)
                                    / f_pdf;
                            }
                        }
                    }
                    l *= lights.len() as Float;
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
