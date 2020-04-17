use crate::{
    geometry::ShapeIntersect,
    scene_file_parser::PropertySet
};

mod bxdf;

mod matte;

pub use matte::*;
pub use bxdf::*;
use std::fmt::Debug;

pub trait Material: Debug + Sync + Send {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> Box<dyn BRDF>;
    fn box_clone(&self) -> Box<dyn Material>;
}

pub fn parse_material(property_set: &PropertySet) -> Box<dyn Material> {
    match property_set.get_name().unwrap() {
        "matte" => {
            let kd = property_set.get_default("Kd");
            Box::new(MatteMaterial::new(kd))
        }
        _ => {
            panic!()
        }
    }
}
