use crate::*;

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
    pub fn evaluate(&self, _uv: &Point2f) -> T {
        self.t.clone()
    }
}

impl<T: ParseFromProperty> ParseFromProperty for Texture<T> {
    fn parse_from_property(property_type: &str, basic_type: &BasicTypes) -> Self {
        Texture::from(T::parse_from_property(property_type, basic_type))
    }
    fn parse_default() -> Self {
        Texture::from(T::parse_default())
    }
}

