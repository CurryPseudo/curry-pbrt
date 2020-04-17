use crate::*;
mod direct_light;
pub use direct_light::*;

pub trait Integrator: Sync {
    fn li(&self, ray: &Ray, scene: &Scene, sampler: &mut dyn Sampler) -> Spectrum;
}

impl ParseFromBlockSegment for Box<dyn Integrator> {
    type T = Box<dyn Integrator>;
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T> {
        let property_set = segment.get_object_by_type("Integrator")?;
        match property_set.get_name().unwrap() {
            "directlighting" => {
                let _max_depth = property_set.get_value("maxdepth").unwrap_or(1);
                Some(Box::new(DirectLightIntegrator::new()))
            }
            _ => panic!(),
        }
    }
}
