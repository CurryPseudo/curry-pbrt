use super::CoefficientSpectrum;
use crate::def::Float;
use derive_more::{Add, Div, Into, Mul, Sub, Index};

#[derive(Clone, Index, Into, Add, Mul, Div, Sub)]
pub struct RGBSpectrum(CoefficientSpectrum);

impl RGBSpectrum {
    pub fn new(v: Float) -> Self {
        Self(CoefficientSpectrum::new(v, 3))
    }
}

impl From<CoefficientSpectrum> for RGBSpectrum {
    fn from(c: CoefficientSpectrum) -> Self {
        assert_eq!(c.len(), 3);
        Self(c)
    }
}

impl From<[Float; 3]> for RGBSpectrum {
    fn from(rgb: [Float; 3]) -> Self {
        Self(CoefficientSpectrum::from(rgb.to_vec().into_boxed_slice()))
    }
}
impl Into<[Float; 3]> for RGBSpectrum {
    fn into(self) -> [Float; 3] {
        [self.0[0], self.0[1], self.0[2]]
    }
}

