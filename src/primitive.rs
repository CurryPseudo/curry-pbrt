use crate::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Primitive {
    shape: Arc<dyn Shape>,
    source: PrimitiveSource,
}

#[derive(Clone)]
pub enum PrimitiveSource {
    Material(Arc<dyn Material>),
    Light(Arc<dyn Light>),
}

impl PrimitiveSource {
    pub fn get_material(&self) -> Option<Arc<dyn Material>> {
        if let Self::Material(m) = self {
            Some(m.clone())
        } else {
            None
        }
    }
    pub fn get_light(&self) -> Option<Arc<dyn Light>> {
        if let Self::Light(l) = self {
            Some(l.clone())
        } else {
            None
        }
    }
    pub fn light(light: Arc<dyn Light>) -> Self {
        Self::Light(light)
    }
    pub fn material(material: Arc<dyn Material>) -> Self {
        Self::Material(material)
    }
}

impl Primitive {
    pub fn new(shape: Arc<dyn Shape>, source: PrimitiveSource) -> Self {
        Self { shape, source }
    }

    pub fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.shape.intersect_predicate(ray)
    }
    pub fn intersect(&self, ray: &Ray) -> Option<PrimitiveIntersect> {
        let shape_intersect = self.shape.intersect(ray)?;
        Some(PrimitiveIntersect::new(shape_intersect, self.clone()))
    }
    pub fn intersect_predicate_through_bound(&self, ray: &RayIntersectCache) -> bool {
        self.shape.intersect_predicate_through_bound(ray)
    }
    pub fn intersect_through_bound(&self, ray: &RayIntersectCache) -> Option<PrimitiveIntersect> {
        let shape_intersect = self.shape.intersect_through_bound(ray)?;
        Some(PrimitiveIntersect::new(shape_intersect, self.clone()))
    }
}
pub struct PrimitiveIntersect {
    shape_intersect: ShapeIntersect,
    primitive: Primitive,
}

impl PrimitiveIntersect {
    pub fn new(shape_intersect: ShapeIntersect, primitive: Primitive) -> Self {
        Self {
            shape_intersect,
            primitive,
        }
    }
    pub fn le(&self) -> Option<Spectrum> {
        self.primitive
            .source
            .get_light()?
            .le(self.shape_intersect.get_shape_point())
    }
    pub fn compute_scattering_functions(&self) -> Option<BSDF> {
        Some(
            self.primitive
                .source
                .get_material()?
                .compute_scattering_functions(&self.shape_intersect),
        )
    }
    pub fn get_shape_intersect(&self) -> &ShapeIntersect {
        &self.shape_intersect
    }
    pub fn get_shape(&self) -> &Arc<dyn Shape> {
        &self.primitive.shape
    }
    pub fn get_light(&self) -> Option<Arc<dyn Light>> {
        self.primitive.source.get_light()
    }
}
