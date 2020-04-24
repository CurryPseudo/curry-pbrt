use crate::*;
use std::sync::Arc;

mod texture_map;

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
    texture_map: texture_map::TextureMap
}

impl SceneParseStack {
    pub fn parse(&mut self, segment: &BlockSegment, scene: &mut Scene) {
        if let Some((_, segments)) = segment.get_block("Attribute") {
            let mut attribute_stack = self.clone();
            for segment in segments {
                attribute_stack.parse(segment, scene);
            }
            return;
        }
        let (object_type, property_set) = segment.get_object().unwrap();
        match object_type {
            "Material" => {
                let m: Arc<dyn Material> = parse_material(property_set, &self.texture_map).into();
                self.material = Some(m.clone());
                scene.materials.push(m);
            }
            "Shape" => {
                for mut shape in parse_shape(property_set) {
                    if let Some(transform) = &self.transform {
                        shape = shape_apply(shape, transform);
                    }
                    let shape: Arc<dyn Shape> = shape.into();
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
                    scene.aggregate.add_primitive(primitive);
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
            _ => {
                let this_transform = Transform::parse_from_segment(segment).unwrap();
                self.transform = Some(
                    self.transform
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
        let mut scene = Scene::default();
        let mut scene_parse_stack = SceneParseStack::default();
        for segment in block_segments {
            scene_parse_stack.parse(segment, &mut scene);
        }
        Some(scene)
    }
}
