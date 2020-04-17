use crate::*;
use std::sync::Arc;

#[derive(Default)]
pub struct Scene {
    lights: Vec<Arc<dyn Light>>,
    materials: Vec<Arc<dyn Material>>,
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
    pub fn intersect(&self, ray: &Ray) -> Option<PrimitiveIntersect> {
        let mut intersect: Option<PrimitiveIntersect> = None;
        let ray = RayIntersectCache::from(*ray);
        for primitive in &self.primitives {
            let this_intersect = primitive.intersect_through_bound(&ray);
            //let this_intersect = primitive.intersect(&ray);
            if let Some(intersect) = &mut intersect {
                if let Some(this_intersect) = this_intersect {
                    if this_intersect.get_shape_intersect().get_t()
                        < intersect.get_shape_intersect().get_t()
                    {
                        *intersect = this_intersect
                    }
                }
            } else {
                intersect = this_intersect;
            }
        }
        intersect
    }
    pub fn get_lights(&self) -> &[Arc<dyn Light>] {
        &self.lights
    }
    pub fn parse_segment(
        &mut self,
        material: &mut Option<Arc<dyn Material>>,
        transform: &mut Option<Transform>,
        area_light_factory: &mut Option<Box<dyn Fn(Arc<dyn Shape>) -> Box<dyn Light>>>,
        segment: &BlockSegment,
    ) {
        if let Some((_, segments)) = segment.get_block("Attribute") {
            let mut material = material.clone();
            let mut transform = transform.clone();
            let mut area_light_factory = None;
            for segment in segments {
                self.parse_segment(
                    &mut material,
                    &mut transform,
                    &mut area_light_factory,
                    segment,
                );
            }
            return;
        }
        let (object_type, property_set) = segment.get_object().unwrap();
        match object_type {
            "Material" => {
                let m: Arc<dyn Material> = parse_material(property_set).into();
                *material = Some(m.clone());
                self.materials.push(m);
            }
            "Shape" => {
                let mut shape = parse_shape(property_set);
                if let Some(transform) = &transform {
                    shape = shape_apply(shape, transform);
                }
                let shape: Arc<dyn Shape> = shape.into();
                let primitive = if let Some(area_light_factory) = area_light_factory {
                    let area_light: Arc<dyn Light> = area_light_factory(shape.clone()).into();
                    self.lights.push(area_light.clone());
                    Primitive::new(shape, PrimitiveSource::light(area_light))
                } else {
                    Primitive::new(shape, PrimitiveSource::material(material.clone().unwrap()))
                };
                self.primitives.push(primitive);
            }
            "LightSource" => {
                let mut light = parse_light(property_set);
                if let Some(transform) = &transform {
                    light = light.box_apply(transform);
                }
                self.lights.push(light.into());
            }
            "AreaLightSource" => {
                *area_light_factory = Some(parse_area_light(property_set));
            }
            _ => {
                let this_transform = Transform::parse_from_segment(segment).unwrap();
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

impl ParseFromBlockSegment for Scene {
    type T = Scene;
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T> {
        let (_, block_segments) = segment.get_block("World")?;
        let mut material = None;
        let mut transform = None;
        let mut area_light_factory = None;
        let mut scene = Scene::default();
        for segment in block_segments {
            scene.parse_segment(
                &mut material,
                &mut transform,
                &mut area_light_factory,
                segment,
            );
        }
        Some(scene)
    }
}
