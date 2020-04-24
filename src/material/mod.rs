use crate::*;
mod bxdf;
mod glass;
mod matte;
mod mirror;

pub use bxdf::*;
pub use glass::*;
pub use matte::*;
pub use mirror::*;
use std::fmt::Debug;

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
        _ => panic!(),
    }
}
