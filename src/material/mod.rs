use crate::geometry::ShapeIntersect;
use bxdf::BRDF;

mod bxdf;

mod matte;

pub use matte::*;

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
pub trait Material {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> Box<dyn BRDF>;
}
