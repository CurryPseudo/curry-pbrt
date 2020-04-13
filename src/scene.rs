use crate::{
    geometry::{Ray, Shape},
    light::Light,
    material::{Material, MaterialIntersect},
    primitive::Primitive,
};

#[derive(Default)]
pub struct Scene {
    lights: Vec<Box<dyn Light>>,
    primitives: Vec<Primitive>,
}

impl Scene {
    pub fn intersect_predicate(&self, ray: &Ray) -> bool {
        for primitive in &self.primitives {
            if primitive.intersect_predicate(ray) {
                return true;
            }
        }
        false
    }
    pub fn intersect(&self, ray: &Ray) -> Option<MaterialIntersect> {
        let mut intersect: Option<MaterialIntersect> = None;
        for primitive in &self.primitives {
            let this_intersect = primitive.intersect(ray);
            if let Some(intersect) = &mut intersect {
                if let Some(this_intersect) = this_intersect {
                    if this_intersect.shape_intersect.t < intersect.shape_intersect.t {
                        *intersect = this_intersect
                    }
                }
            } else {
                intersect = this_intersect;
            }
        }
        intersect
    }
    pub fn get_lights(&self) -> &[Box<dyn Light>] {
        &self.lights
    }
    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }
    pub fn add_primitive(&mut self, primitive: Primitive)  {
        self.primitives.push(primitive);
    }
}
