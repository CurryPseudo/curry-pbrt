use crate::*;
use std::sync::Arc;
mod constant;
mod image;
pub use constant::*;
pub use image::*;

pub trait Texture<T>: std::fmt::Debug + Send + Sync {
    fn evaluate(&self, uv: &Point2f) -> T;
    fn pixels(&self) -> FixedVec2D<T>;
}

pub trait TextureMap: std::fmt::Debug {
    fn get<T: Send + Sync + 'static>(&self, name: &str) -> Option<Arc<ImageTexture<T>>>;
}

impl TextureMap for () {
    fn get<T>(&self, _: &str) -> Option<Arc<ImageTexture<T>>> {
        None
    }
}

pub enum TextureParseResult<T> {
    Value(Arc<dyn Texture<T>>),
    FromName(String),
}

impl<T: ImageTextureContent + Send + Sync + std::fmt::Debug + Clone + 'static>
    TextureParseResult<T>
{
    pub fn into_texture<M: TextureMap>(self, map: &M) -> Arc<dyn Texture<T>> {
        match self {
            Self::Value(r) => r,
            Self::FromName(name) => map.get(&name).unwrap(),
        }
    }
}

impl<T: std::fmt::Debug + Sync + Send + Clone + 'static + ParseFromProperty> ParseFromProperty
    for TextureParseResult<T>
{
    fn parse_from_property(property_type: &str, basic_type: &BasicTypes) -> Self {
        match property_type {
            "texture" => {
                TextureParseResult::FromName(String::parse_from_property(property_type, basic_type))
            }
            _ => TextureParseResult::Value(Arc::new(ConstantTexture::from(
                T::parse_from_property(property_type, basic_type),
            ))),
        }
    }
    fn parse_default() -> Self {
        TextureParseResult::Value(Arc::new(ConstantTexture::from(T::parse_default())))
    }
}

impl<T: 'static + std::fmt::Debug + Sync + Send + Clone + ImageTextureContent> ParseFromProperty
    for Arc<dyn Texture<T>>
{
    fn parse_from_property(type_name: &str, basic_type: &BasicTypes) -> Self {
        match type_name {
            "string" => {
                let texture_path = basic_type.get_path().unwrap();
                Arc::new(ImageTexture::from_file(&texture_path))
            }
            _ => panic!()
        }
    }
    fn parse_default() -> Self {
        constant_texture(T::default())
    }
}

pub fn constant_texture<T: 'static + std::fmt::Debug + Sync + Send + Clone>(
    t: T,
) -> Arc<dyn Texture<T>> {
    Arc::new(ConstantTexture::from(t))
}

fn option_to_texture<
    T: ImageTextureContent + Send + Sync + std::fmt::Debug + Clone + 'static,
    M: TextureMap,
>(
    texture_parse_result: Option<TextureParseResult<T>>,
    m: &M,
) -> Option<Arc<dyn Texture<T>>> {
    texture_parse_result.map(|texture_parse_result| texture_parse_result.into_texture(m))
}

pub fn get_texture<
    T: ImageTextureContent + 'static + Send + Sync + std::fmt::Debug + ParseFromProperty + Clone,
    M: TextureMap,
>(
    property_set: &PropertySet,
    name: &str,
    map: &M,
) -> Option<Arc<dyn Texture<T>>> {
    option_to_texture(property_set.get_value(name), map)
}
