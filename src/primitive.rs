use crate::{
    geometry::{Ray, RayIntersectCache, Shape},
    material::{Material, MaterialIntersect},
};

pub struct Primitive {
    shape: Box<dyn Shape>,
    material: Box<dyn Material>,
}

impl Primitive {
    pub fn new(shape: Box<dyn Shape>, material: Box<dyn Material>) -> Self {
        Self { shape, material }
    }

    pub fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.shape.intersect_predicate(ray)
    }
    pub fn intersect(&self, ray: &Ray) -> Option<MaterialIntersect> {
        let shape_intersect = self.shape.intersect(ray)?;
        Some(MaterialIntersect::new(
            shape_intersect,
            self.material.as_ref(),
        ))
    }
    pub fn intersect_predicate_through_bound(&self, ray: &RayIntersectCache) -> bool {
        self.shape.intersect_predicate_through_bound(ray)
    }
    pub fn intersect_through_bound(&self, ray: &RayIntersectCache) -> Option<MaterialIntersect> {
        let shape_intersect = self.shape.intersect_through_bound(ray)?;
        Some(MaterialIntersect::new(
            shape_intersect,
            self.material.as_ref(),
        ))
    }
}
