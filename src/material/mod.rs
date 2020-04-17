use crate::*;
mod bxdf;

mod matte;

pub use matte::*;
pub use bxdf::*;
use std::fmt::Debug;

pub trait Material: Debug + Sync + Send {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> Box<dyn BxDF>;
    fn box_clone(&self) -> Box<dyn Material>;
}

pub fn parse_material(property_set: &PropertySet) -> Box<dyn Material> {
    match property_set.get_name().unwrap() {
        "matte" => {
            let kd = property_set.get_value("Kd").unwrap_or(Texture::from(Spectrum::new(0.5)));
            let sigma = property_set.get_value("sigma").unwrap_or(Texture::from(0.));
            Box::new(MatteMaterial::new(kd, sigma))
        }
        _ => {
            panic!()
        }
    }
}
