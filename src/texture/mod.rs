use crate::*;
use std::sync::Arc;
mod constant;
mod image;
pub use constant::*;
pub use image::*;

pub trait Texture<T>: std::fmt::Debug + Send + Sync {
    fn evaluate(&self, uv: &Point2f) -> T;

}


impl<T: std::fmt::Debug + Sync + Send + Clone + 'static + ParseFromProperty> ParseFromProperty for Arc<dyn Texture<T>> {
    fn parse_from_property(property_type: &str, basic_type: &BasicTypes) -> Self {
        Arc::new(ConstantTexture::from(T::parse_from_property(
            property_type,
            basic_type,
        )))
    }
    fn parse_default() -> Self {
        Arc::new(ConstantTexture::from(T::parse_default()))
    }
}

pub fn constant_texture<T: 'static + std::fmt::Debug + Sync + Send + Clone>(t: T) -> Arc<dyn Texture<T>> {
    Arc::new(ConstantTexture::from(t))
}
