use crate::{material::{MaterialIntersect, Material}, geometry::{Ray, Shape}};

pub struct Primitive {
    shape: Box<dyn Shape>,
    material: Box<dyn Material>
}

impl Primitive {
    pub fn new(shape: Box<dyn Shape>, material: Box<dyn Material>) -> Self { Self { shape, material } }

    pub fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.shape.intersect_predicate(ray)
    }
    pub fn intersect(&self, ray: &Ray) -> Option<MaterialIntersect> {
        let shape_intersect = self.shape.intersect(ray)?;
        Some(MaterialIntersect::new(shape_intersect, self.material.as_ref()))
    }
}
