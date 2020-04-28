use crate::*;
use std::collections::HashMap;
use std::sync::Arc;

mod clipper;
mod texture_map;
pub use clipper::*;

#[derive(Default)]
pub struct Scene {
    lights: Vec<Arc<dyn Light>>,
    materials: Vec<Arc<dyn Material>>,
    aggregate: AggregateBuilder,
}

impl Scene {
    pub fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.aggregate.get().intersect_predicate(ray)
    }
    pub fn intersect(&self, ray: &Ray) -> Option<PrimitiveIntersect> {
        self.aggregate.get().intersect(&ray)
    }
    pub fn get_lights(&self) -> &[Arc<dyn Light>] {
        &self.lights
    }
    pub fn build_aggregate(&mut self, aggregate: Box<dyn Aggregate>) {
        self.aggregate.build(aggregate);
    }
}

#[derive(Default, Clone)]
struct SceneParseStack {
    material: Option<Arc<dyn Material>>,
    transform: Option<Transform>,
    area_light_factory: Option<AreaLightFactory>,
    texture_map: texture_map::TextureMap,
    named_material: HashMap<String, Arc<dyn Material>>,
    object_name: Option<String>,
}

impl SceneParseStack {
    pub fn parse(
        &mut self,
        segment: &BlockSegment,
        scene: &mut Scene,
        objects: &mut HashMap<String, Vec<Primitive>>,
        clipper: Option<&dyn PrimitiveClipper>,
    ) {
        if let Some((block_type, block_name, segments)) = segment.as_block() {
            match block_type {
                "Attribute" => {
                    let mut attribute_stack = self.clone();
                    for segment in segments {
                        attribute_stack.parse(segment, scene, objects, clipper);
                    }
                }
                "Object" => {
                    let mut object_stack = self.clone();
                    object_stack.object_name = Some(block_name.as_ref().unwrap().clone());
                    for segment in segments {
                        object_stack.parse(segment, scene, objects, clipper);
                    }
                }
                _ => panic!(),
            }
            return;
        }
        let (object_type, property_set) = segment.get_object().unwrap();
        match object_type {
            "MakeNamedMaterial" => {
                let (name, m) = parse_make_named_material(
                    property_set,
                    &self.texture_map,
                    &self.named_material,
                );
                self.named_material.insert(String::from(name), m.into());
            }
            "Material" => {
                let m: Arc<dyn Material> =
                    parse_material(property_set, &self.texture_map, &self.named_material).into();
                self.material = Some(m.clone());
                scene.materials.push(m);
            }
            "Shape" => {
                for mut shape in parse_shape(property_set) {
                    if let Some(transform) = &self.transform {
                        shape = shape_apply(shape, transform);
                    }
                    let primitive = if let Some(area_light_factory) = &self.area_light_factory {
                        let area_light: Arc<dyn Light> = area_light_factory(shape.clone()).into();
                        scene.lights.push(area_light.clone());
                        Primitive::new(shape, PrimitiveSource::light(area_light))
                    } else {
                        Primitive::new(
                            shape,
                            PrimitiveSource::material(self.material.clone().unwrap()),
                        )
                    };
                    if let Some(object_name) = &self.object_name {
                        objects
                            .entry(object_name.clone())
                            .or_default()
                            .push(primitive);
                    } else {
                        let mut clip = false;
                        if let Some(clipper) = clipper {
                            if clipper.clip(&primitive) {
                                clip = true;
                            }
                        }
                        if !clip {
                            scene.aggregate.add_primitive(primitive);
                        }
                    }
                }
            }
            "ObjectInstance" => {
                let object_name = property_set.get_name().unwrap();
                if let Some(primitives) = objects.get(object_name) {
                    for primitive in primitives {
                        let mut primitive = primitive.clone();
                        if let Some(transform) = &self.transform {
                            primitive = primitive.apply(transform);
                        }
                        let mut clip = false;
                        if let Some(clipper) = clipper {
                            if clipper.clip(&primitive) {
                                clip = true;
                            }
                        }
                        if !clip {
                            scene.aggregate.add_primitive(primitive);
                        }
                    }
                }
            }
            "LightSource" => {
                let mut light = parse_light(property_set);
                if let Some(transform) = &self.transform {
                    light = light.box_apply(transform);
                }
                scene.lights.push(light.into());
            }
            "AreaLightSource" => {
                self.area_light_factory = Some(parse_area_light(property_set));
            }
            "Texture" => {
                self.texture_map.add_texture(property_set);
            }
            "Transform" => {
                self.transform = Some(Transform::parse_from_segment(segment).unwrap());
            }
            _ => {
                if let Some(this_transform) = Transform::parse_from_segment(segment) {
                    self.transform = Some(
                        self.transform
                            .clone()
                            .map_or(this_transform.clone(), |transform| {
                                transform.apply(&this_transform)
                            }),
                    );
                } else {
                    error!("Parsing scene error");
                    error!("object_type {}", object_type);
                    error!("property_set {:#?}", property_set);
                }
            }
        }
    }
}

pub struct SceneBuilder<'a> {
    segments: &'a Vec<BlockSegment>,
}

impl<'a> SceneBuilder<'a> {
    pub fn build_with_clipper(&self, clipper: Option<&dyn PrimitiveClipper>) -> Scene {
        let mut scene = Scene::default();
        let mut scene_parse_stack = SceneParseStack::default();
        let mut objects = HashMap::new();
        for segment in self.segments {
            scene_parse_stack.parse(segment, &mut scene, &mut objects, clipper);
        }
        scene
    }
}

impl<'a> ParseFromBlockSegment<'a> for SceneBuilder<'a> {
    type T = SceneBuilder<'a>;
    fn parse_from_segment(segment: &'a BlockSegment) -> Option<Self::T> {
        let (_, block_segments) = segment.get_block("World")?;
        Some(SceneBuilder{segments: block_segments})

    }
}
