use crate::*;
pub struct PathIntegrator {
    max_depth: usize,
}

impl PathIntegrator {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }
}

impl Integrator for PathIntegrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Spectrum {
        let mut l = Spectrum::new(0.);
        let mut beta = Spectrum::new(1.);
        let mut ray = *ray;
        let mut bounce = 0;
        let mut specular_bounce = false;
        loop {
            let intersect = scene.intersect(&ray);
            if bounce == 0 || specular_bounce {
                if let Some(intersect) = &intersect {
                    l += beta * intersect.le();
                } else {
                    for light in scene.get_lights() {
                        l += beta * light.le_out_scene(&ray);
                    }
                }
            }
            if let Some(intersect) = intersect {
                if bounce >= self.max_depth {
                    break;
                }
                let shape_point = intersect.get_shape_point();
                if let Some(bsdf) = intersect.compute_scattering_functions() {
                    let wo = -ray.d;
                    if !bsdf.is_all_delta() {
                        l += beta
                            * uniform_sample_one_light(shape_point, &bsdf, &wo, scene, sampler);
                    }
                    if let (wi, Some(f), f_pdf, is_delta) = bsdf.sample_f(&wo, sampler) {
                        if f_pdf != 0. {
                            beta *= f * wi.dot(&shape_point.n).abs() / f_pdf;
                            ray = Ray::new_shape_point_d(shape_point, wi);
                            specular_bounce = is_delta;

                            if bounce > 3 {
                                let q = max(0.05, 1. - beta.y());
                                if sampler.get_1d() < q {
                                    break;
                                }
                                beta /= 1. - q;
                            }
                            bounce += 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
        l
    }
}
