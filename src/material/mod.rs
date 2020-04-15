use crate::{
    geometry::ShapeIntersect,
    scene_file_parser::PropertySet
};
use bxdf::BRDF;

mod bxdf;

mod matte;

pub use matte::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct MaterialIntersect<'a> {
    pub shape_intersect: ShapeIntersect,
    pub material: &'a dyn Material,
}
impl<'a> MaterialIntersect<'a> {
    pub fn new(shape_intersect: ShapeIntersect, material: &'a dyn Material) -> Self {
        Self {
            shape_intersect,
            material,
        }
    }

    pub fn compute_scattering_functions(&self) -> Box<dyn BRDF> {
        self.material
            .compute_scattering_functions(&self.shape_intersect)
    }
}
pub trait Material: Debug + Sync {
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
