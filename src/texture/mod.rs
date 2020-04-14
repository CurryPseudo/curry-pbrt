use crate::{
    geometry::Point2f,
    scene_file_parser::{BasicTypes, ParseFromBasicType, PropertySet},
    spectrum::Spectrum,
};

#[derive(Debug, Clone)]
pub struct Texture<T> {
    t: T,
}

impl<T> From<T> for Texture<T> {
    fn from(t: T) -> Self {
        Self { t }
    }
}

impl<T: Clone> Texture<T> {
    pub fn evaluate(&self, uv: &Point2f) -> T {
        self.t.clone()
    }
}

pub fn parse_spectrum_texture(
    (texture_type, texture_value): (&String, &BasicTypes),
) -> Texture<Spectrum> {
    if texture_type == "rgb" {
        let s = Spectrum::parse_from_basic_type(texture_value);
        return Texture::from(s);
    }
    panic!()
}
pub fn parse_spectrum_texture_default(property_set: &PropertySet, name: &str) -> Texture<Spectrum> {
    property_set.get_typed_value(name).map_or(
        Texture::from(Spectrum::from([1., 1., 1.])),
        |type_and_value| parse_spectrum_texture(type_and_value),
    )
}
