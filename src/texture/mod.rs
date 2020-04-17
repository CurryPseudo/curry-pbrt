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

impl ParseFromProperty for Texture<Spectrum> {
    fn parse_from_property(property_type: &str, basic_type: &BasicTypes) -> Self {
        match property_type {
            "rgb" => {
                Texture::from(Spectrum::parse_from_property(property_type, basic_type))
            }
            _ => panic!()
        }
    }
    fn parse_default() -> Self {
        Texture::from(Spectrum::parse_default())
    }
}

