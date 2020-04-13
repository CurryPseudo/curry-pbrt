use crate::{geometry::Ray, spectrum::Spectrum, scene::Scene, sampler::SamplerWrapper};

mod direct_light;
pub use direct_light::*;

pub trait Integrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut SamplerWrapper) -> Spectrum;
}
