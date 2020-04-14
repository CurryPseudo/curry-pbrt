mod rgb_spectrum;
pub use rgb_spectrum::*;
use crate::scene_file_parser::{PropertySet, ParseFromBasicType};

pub type Spectrum = RGBSpectrum;

pub fn parse_spectrum_default(property_set: &PropertySet, name: &str) -> Spectrum {
    property_set.get_typed_value(name).map_or(Spectrum::from([1., 1., 1.]), |(_, basic_types)| Spectrum::parse_from_basic_type(basic_types))
}
