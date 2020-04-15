use crate::{geometry::Ray, sampler::SamplerWrapper, scene::Scene, spectrum::Spectrum, scene_file_parser::BlockSegment};

mod direct_light;
pub use direct_light::*;

pub trait Integrator {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut SamplerWrapper) -> Spectrum;
}

pub fn parse_integrator(segment: &BlockSegment) -> Option<Box<dyn Integrator>> {
    let property_set = segment.get_object_by_type("Integrator")?;
    match property_set.get_name().unwrap() {
        "directlighting" => {
            let max_depth = property_set.get_value("maxdepth").unwrap_or(1);
            Some(Box::new(DirectLightIntegrator::new()))
        }
        _ => panic!()
    }
}
