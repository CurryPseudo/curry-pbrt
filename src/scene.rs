use crate::{
    geometry::{
        parse_shape, parse_transform, shape_apply, Ray, RayIntersectCache, Transform,
        TransformShape, Transformable,
    },
    light::{parse_light, Light},
    material::{parse_material, Material, MaterialIntersect},
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
        let ray = RayIntersectCache::from(*ray);
        for primitive in &self.primitives {
            if primitive.intersect_predicate_through_bound(&ray) {
            //if primitive.intersect_predicate(&ray) {
                return true;
            }
        }
        false
    }
    pub fn intersect(&self, ray: &Ray) -> Option<MaterialIntersect> {
        let mut intersect: Option<MaterialIntersect> = None;
        let ray = RayIntersectCache::from(*ray);
        for primitive in &self.primitives {
            let this_intersect = primitive.intersect_through_bound(&ray);
            //let this_intersect = primitive.intersect(&ray);
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
    pub fn parse_segment(
        &mut self,
        material: &mut Option<Box<dyn Material>>,
        transform: &mut Option<Transform>,
        segment: &BlockSegment,
    ) {
        if let Some((_, segments)) = segment.get_block("Attribute") {
            let mut material = material.as_ref().map(|material| material.box_clone());
            let mut transform = transform.clone();
            for segment in segments {
                self.parse_segment(&mut material, &mut transform, segment);
            }
            return;
        }
        let (object_type, property_set) = segment.get_object().unwrap();
        match object_type {
            "Material" => {
                *material = Some(parse_material(property_set));
            }
            "Shape" => {
                let mut shape = parse_shape(property_set);
                if let Some(transform) = &transform {
                    shape = shape_apply(shape, transform);
                }
                self.add_primitive(Primitive::new(
                    shape,
                    material.as_ref().unwrap().box_clone(),
                ))
            }
            "LightSource" => {
                let mut light = parse_light(property_set);
                if let Some(transform) = &transform {
                    light = light.box_apply(transform);
                }
                self.add_light(light);
            }
            _ => {
                let this_transform = parse_transform(segment).unwrap();
                *transform = Some(
                    transform
                        .clone()
                        .map_or(this_transform.clone(), |transform| {
                            transform.apply(&this_transform)
                        }),
                );
            }
        }
    }
}

pub fn parse_scene(segment: &BlockSegment) -> Scene {
    let (_, block_segments) = segment.get_block("World").unwrap();
    let mut material = None;
    let mut transform = None;
    let mut scene = Scene::default();
    for segment in block_segments {
        scene.parse_segment(&mut material, &mut transform, segment);
    }
    scene
}
