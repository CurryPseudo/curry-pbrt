use crate::{
    geometry::{parse_shape, parse_transform, Ray, TransformShape, Transformable, shape_apply},
    light::{parse_light, Light},
    material::{parse_material, MaterialIntersect},
    primitive::Primitive,
    scene_file_parser::BlockSegment,
};
use std::collections::VecDeque;

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
    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

pub fn parse_scene(segment: &BlockSegment) -> Scene {
    let (_, block_segments) = segment.get_block("World").unwrap();
    let mut material = None;
    let mut transform = None;
    let mut scene = Scene::default();
    for segment in block_segments {
        let (object_type, property_set) = segment.get_object().unwrap();
        match object_type {
            "Material" => {
                material = Some(parse_material(property_set));
            }
            "Shape" => {
                let mut shape = parse_shape(property_set);
                if let Some(transform) = &transform {
                    shape = shape_apply(shape, transform);
                }
                scene.add_primitive(Primitive::new(
                    shape,
                    material.as_ref().unwrap().box_clone(),
                ))
            }
            "LightSource" => {
                let mut light = parse_light(property_set);
                if let Some(transform) = &transform {
                    light = light.box_apply(transform);
                }
                scene.add_light(light);
            }
            _ => {
                transform = Some(parse_transform(segment).unwrap());
            }
        }
    }
    scene
}
