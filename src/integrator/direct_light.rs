use super::Integrator;
use crate::*;
use std::sync::Arc;

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
            if let Some(bsdf) = intersect.compute_scattering_functions() {
                let lights = scene.get_lights();
                if lights.len() > 0 {
                    let wo = -ray.d.normalize();
                    let light = &lights[(sampler.get_usize(lights.len())) as usize];
                    let shape_point = &intersect.get_shape_intersect().get_shape_point();
                    let point = shape_point.p.clone();
                    let n = &intersect.get_shape_intersect().get_normal();
                    {
                        // sample light
                        if let (wi, Some(li), li_pdf, visibility_tester) =
                            light.sample_li(&shape_point, sampler)
                        {
                            if li_pdf > 0. && visibility_tester.unoccluded(scene) {
                                trace!("Sample light Get li {} pdf {}", li, li_pdf);
                                if let (Some(f), f_pdf) = bsdf.f_pdf(&wo, &wi) {
                                    trace!("Sample light Get f {} {}", f, f_pdf);
                                    if light.is_delta() {
                                        l += li * f * n.dot(&wi).abs() / li_pdf;
                                    } else {
                                        l += li
                                            * f
                                            * n.dot(&wi).abs()
                                            * power_heuristic(li_pdf, f_pdf)
                                            / li_pdf;
                                    }
                                }
                            }
                        }
                    }
                    if !light.is_delta() {
                        // sample brdf
                        if let (wi, Some(f), f_pdf) = bsdf.sample_f(&wo, sampler) {
                            trace!("Sample bsdf Get f {} pdf {}", f, f_pdf);
                            let ray = Ray::new_shape_point_d(&shape_point, wi.clone());
                            if let Some(intersect) = scene.intersect(&ray) {
                                if let Some(intersect_light) = intersect.get_light() {
                                    if Arc::ptr_eq(light, &intersect_light) {
                                        if let (Some(li), li_pdf) = light.le_pdf(
                                            &point,
                                            intersect.get_shape_intersect().get_shape_point(),
                                        ) {
                                            trace!("Sample bsdf Get li {} pdf {}", li, f_pdf);
                                            l += li
                                                * f
                                                * n.dot(&wi).abs()
                                                * power_heuristic(f_pdf, li_pdf)
                                                / f_pdf;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                l *= lights.len() as Float;
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
