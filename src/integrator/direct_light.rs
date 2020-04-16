use super::Integrator;
use crate::{
    def::Float, geometry::Ray, math::power_heuristic, sampler::SamplerWrapper, scene::Scene,
    spectrum::Spectrum,
};

pub struct DirectLightIntegrator {}

impl DirectLightIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for DirectLightIntegrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut SamplerWrapper) -> Spectrum {
        let mut l = Spectrum::new(0.);
        if let Some(intersect) = scene.intersect(ray) {
            if ray.d.x == 0. && ray.d.y == 0. {
                trace!("Hit with {:?}", intersect);
            }
            let wo = -ray.d.normalize();
            let lights = scene.get_lights();
            let light = &lights[(sampler.get_1d() * lights.len() as Float) as usize];
            let brdf = intersect.compute_scattering_functions();
            let point = &intersect.shape_intersect.p;
            let n = &intersect.shape_intersect.n;
            {
                // sample light
                let li_pdf = light.sample_li_with_visibility_test(
                    &intersect.shape_intersect.p,
                    sampler,
                    scene,
                );
                if let (wi, Some(li)) = li_pdf.t {
                    let f_pdf = brdf.f_pdf(&wo, &wi, n);
                    if let Some(f) = f_pdf.t {
                        if light.is_delta() {
                            l += li * f * n.dot(&wi).abs() / li_pdf.pdf;
                        } else {
                            l += li * f * n.dot(&wi).abs() * power_heuristic(li_pdf.pdf, f_pdf.pdf)
                                / li_pdf.pdf;
                        }
                    }
                }
            }
            if !light.is_delta() {
                // sample brdf
                let f_pdf = brdf.sample_f(&wo, sampler);

                if let (wi, Some(f)) = f_pdf.t {
                    let ray = Ray::new_od(point.clone(), wi.clone());
                    let li_pdf = light.le_pdf(&ray);
                    if let Some(li) = li_pdf.t {
                        l += li * f * n.dot(&wi).abs() * power_heuristic(f_pdf.pdf, li_pdf.pdf)
                            / f_pdf.pdf;
                    }
                }
            }
            l *= lights.len() as Float;
        } else {
            for light in scene.get_lights() {
                l += light.le(ray);
            }
        }
        l
    }
}
