use crate::*;
use std::collections::HashMap;
use std::sync::Arc;
mod bxdf;
mod glass;
mod matte;
mod mirror;
mod translucent;
mod uber;
mod mix;

pub use bxdf::*;
pub use glass::*;
pub use matte::*;
pub use mirror::*;
use std::fmt::Debug;
pub use translucent::*;
pub use uber::*;
pub use mix::*;

pub trait Material: Debug + Sync + Send {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> BSDF;
    fn box_clone(&self) -> Box<dyn Material>;
}

pub fn parse_material<M: TextureMap>(
    property_set: &PropertySet,
    texture_map: &M,
    material_map: &HashMap<String, Arc<dyn Material>>,
) -> Box<dyn Material> {
    parse_material_with_type(
        property_set.get_name().unwrap(),
        property_set,
        texture_map,
        material_map,
    )
}
pub fn parse_make_named_material<'a, M: TextureMap>(
    property_set: &'a PropertySet,
    map: &M,
    material_map: &HashMap<String, Arc<dyn Material>>,
) -> (&'a str, Box<dyn Material>) {
    let material_type: String = property_set.get_value("type").unwrap();
    let name = property_set.get_name().unwrap();
    (
        name,
        parse_material_with_type(&material_type, property_set, map, material_map),
    )
}
pub fn parse_material_with_type<M: TextureMap>(
    material_type: &str,
    property_set: &PropertySet,
    texture_map: &M,
    material_map: &HashMap<String, Arc<dyn Material>>,
) -> Box<dyn Material> {
    match material_type {
        "matte" => {
            let kd = get_texture(property_set, "Kd", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let sigma = get_texture(property_set, "sigma", texture_map)
                .unwrap_or_else(|| constant_texture(0.));
            Box::new(MatteMaterial::new(kd, sigma))
        }
        "glass" => {
            let r = get_texture(property_set, "Kr", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let t = get_texture(property_set, "Kt", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(1.)));
            let eta = get_texture(property_set, "index", texture_map)
                .unwrap_or_else(|| constant_texture(1.5));
            Box::new(GlassMaterial::new(r, t, eta))
        }
        "mirror" => {
            let r = get_texture(property_set, "Kr", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(1.)));
            Box::new(MirrorMaterial::new(r))
        }
        "uber" => {
            let kd = get_texture(property_set, "Kd", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let ks = get_texture(property_set, "Ks", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let kr = get_texture(property_set, "Kr", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.)));
            let kt = get_texture(property_set, "Kt", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.)));
            let roughness = get_texture(property_set, "roughness", texture_map)
                .unwrap_or_else(|| constant_texture(0.1));
            let uroughness = get_texture(property_set, "uroughness", texture_map);
            let vroughness = get_texture(property_set, "uroughness", texture_map);
            let eta = get_texture(property_set, "eta", texture_map).unwrap_or_else(|| {
                get_texture(property_set, "index", texture_map)
                    .unwrap_or_else(|| constant_texture(1.5))
            });
            let opacity = get_texture(property_set, "opacity", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(1.)));
            Box::new(UberMaterial {
                kd,
                ks,
                kr,
                kt,
                roughness,
                uroughness,
                vroughness,
                eta,
                opacity,
            })
        }
        "translucent" => {
            let kd = get_texture(property_set, "Kd", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let ks = get_texture(property_set, "Ks", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let reflect = get_texture(property_set, "reflect", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let transmit = get_texture(property_set, "transmit", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let roughness = get_texture(property_set, "roughness", texture_map)
                .unwrap_or_else(|| constant_texture(0.1));
            Box::new(TranslucentMaterial {
                kd,
                ks,
                reflect,
                transmit,
                roughness,
            })
        }
        "mix" => {
            let scale = get_texture(property_set, "amount", texture_map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.)));
            let namedmaterial1: String = property_set.get_value("namedmaterial1").unwrap();
            let m1 = material_map.get(&namedmaterial1).unwrap().clone();
            let namedmaterial2: String = property_set.get_value("namedmaterial2").unwrap();
            let m2 = material_map.get(&namedmaterial2).unwrap().clone();
            Box::new(MixMaterial{m1, m2, scale})
        }
        _ => panic!(),
    }
}
