use crate::*;
mod bxdf;
mod glass;
mod matte;
mod mirror;
mod translucent;
mod uber;

pub use bxdf::*;
pub use glass::*;
pub use matte::*;
pub use mirror::*;
use std::fmt::Debug;
pub use translucent::*;
pub use uber::*;

pub trait Material: Debug + Sync + Send {
    fn compute_scattering_functions(&self, shape_intersect: &ShapeIntersect) -> BSDF;
    fn box_clone(&self) -> Box<dyn Material>;
}

pub fn parse_material<M: TextureMap>(property_set: &PropertySet, map: &M) -> Box<dyn Material> {
    match property_set.get_name().unwrap() {
        "matte" => {
            let kd = get_texture(property_set, "Kd", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let sigma =
                get_texture(property_set, "sigma", map).unwrap_or_else(|| constant_texture(0.));
            Box::new(MatteMaterial::new(kd, sigma))
        }
        "glass" => {
            let r = get_texture(property_set, "Kr", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let t = get_texture(property_set, "Kt", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(1.)));
            let eta =
                get_texture(property_set, "index", map).unwrap_or_else(|| constant_texture(1.5));
            Box::new(GlassMaterial::new(r, t, eta))
        }
        "mirror" => {
            let r = get_texture(property_set, "Kr", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(1.)));
            Box::new(MirrorMaterial::new(r))
        }
        "uber" => {
            let kd = get_texture(property_set, "Kd", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let ks = get_texture(property_set, "Ks", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let kr = get_texture(property_set, "Kr", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.)));
            let kt = get_texture(property_set, "Kt", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.)));
            let roughness = get_texture(property_set, "roughness", map)
                .unwrap_or_else(|| constant_texture(0.1));
            let uroughness = get_texture(property_set, "uroughness", map);
            let vroughness = get_texture(property_set, "uroughness", map);
            let eta = get_texture(property_set, "eta", map).unwrap_or_else(|| {
                get_texture(property_set, "index", map).unwrap_or_else(|| constant_texture(1.5))
            });
            let opacity = get_texture(property_set, "opacity", map)
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
            let kd = get_texture(property_set, "Kd", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let ks = get_texture(property_set, "Ks", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.25)));
            let reflect = get_texture(property_set, "reflect", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let transmit = get_texture(property_set, "transmit", map)
                .unwrap_or_else(|| constant_texture(Spectrum::new(0.5)));
            let roughness = get_texture(property_set, "roughness", map)
                .unwrap_or_else(|| constant_texture(0.1));
            Box::new(TranslucentMaterial {
                kd,
                ks,
                reflect,
                transmit,
                roughness,
            })
        }
        _ => panic!(),
    }
}
