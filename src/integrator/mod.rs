use crate::*;
mod direct_light;
mod path;
pub use direct_light::*;
pub use path::*;
use std::sync::Arc;

pub trait Integrator: Sync {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Spectrum;
}

pub fn uniform_sample_one_light(
    shape_point: &ShapePoint,
    bsdf: &BSDF,
    wo: &Vector3f,
    scene: &Scene,
    sampler: &mut dyn Sampler,
) -> Spectrum {
    let mut l = Spectrum::new(0.);
    let lights = scene.get_lights();
    let n = &shape_point.n;
    if lights.len() > 0 {
        let light = &lights[(sampler.get_usize(lights.len())) as usize];
        let point = shape_point.p.clone();
        {
            // sample light
            if let (wi, Some(li), li_pdf, visibility_tester) =
                light.sample_li(&shape_point, sampler)
            {
                trace!("Sample light Get li {} pdf {}", li, li_pdf);
                if let (Some(f), f_pdf) = bsdf.f_pdf(&wo, &wi) {
                    if f_pdf != 0. {
                        if visibility_tester.unoccluded(scene) {
                            if light.is_delta() {
                                l += li * f * n.dot(&wi).abs() / li_pdf;
                            } else {
                                l += li * f * n.dot(&wi).abs() * power_heuristic(li_pdf, f_pdf)
                                    / li_pdf;
                            }
                        }
                        trace!("Sample light Get f {} {}", f, f_pdf);
                    }
                }
            }
        }
        if !light.is_delta() {
            // sample brdf
            if let (wi, Some(f), f_pdf) = bsdf.sample_no_delta_f(&wo, &sampler.get_2d()) {
                trace!("Sample bsdf Get f {} pdf {}", f, f_pdf);
                let ray = Ray::new_shape_point_d(&shape_point, wi.clone());
                if let Some(intersect) = scene.intersect(&ray) {
                    if let Some(intersect_light) = intersect.get_light() {
                        if Arc::ptr_eq(light, &intersect_light) {
                            if let (Some(li), li_pdf) = light
                                .le_pdf(&point, intersect.get_shape_point())
                            {
                                trace!("Sample bsdf Get li {} pdf {}", li, f_pdf);
                                l += li * f * n.dot(&wi).abs() * power_heuristic(f_pdf, li_pdf)
                                    / f_pdf;
                            }
                        }
                    }
                }
            }
        }
    }
    l *= lights.len() as Float;
    l
}
impl ParseFromBlockSegment for Box<dyn Integrator> {
    type T = Box<dyn Integrator>;
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T> {
        let property_set = segment.get_object_by_type("Integrator")?;
        match property_set.get_name().unwrap() {
            "directlighting" => {
                let max_depth = property_set.get_value("maxdepth").unwrap_or(5);
                Some(Box::new(DirectLightIntegrator::new(max_depth)))
            }
            "path" => {
                let max_depth = property_set.get_value("maxdepth").unwrap_or(5);
                Some(Box::new(PathIntegrator::new(max_depth)))
            }
            _ => panic!(),
        }
    }
}

