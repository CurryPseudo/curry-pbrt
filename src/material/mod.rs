use crate::*;
mod matte;
mod bxdf;
mod glass;
mod mirror;

pub use matte::*;
pub use bxdf::*;
pub use glass::*;
pub use mirror::*;
use std::fmt::Debug;

pub trait Material: Debug + Sync + Send {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> BSDF;
    fn box_clone(&self) -> Box<dyn Material>;
}

pub fn parse_material(property_set: &PropertySet) -> Box<dyn Material> {
    match property_set.get_name().unwrap() {
        "matte" => {
            let kd = property_set.get_value("Kd").unwrap_or(Texture::from(Spectrum::new(0.5)));
            let sigma = property_set.get_value("sigma").unwrap_or(Texture::from(0.));
            Box::new(MatteMaterial::new(kd, sigma))
        }
        "glass" => {
            let r = property_set.get_value("Kr").unwrap_or(Texture::from(Spectrum::new(1.)));
            let t = property_set.get_value("Kt").unwrap_or(Texture::from(Spectrum::new(1.)));
            let eta = property_set.get_value("index").unwrap_or(Texture::from(1.5));
            Box::new(GlassMaterial::new(r, t, eta))
        }
        "mirror" => {
            let r = property_set.get_value("Kr").unwrap_or(Texture::from(Spectrum::new(1.)));
            Box::new(MirrorMaterial::new(r))
        }
        _ => {
            panic!()
        }
    }
}
