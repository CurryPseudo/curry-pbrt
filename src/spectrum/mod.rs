use crate::{def::Float, math::clamp};
use ops::Index;
use std::ops;

#[derive(Clone)]
pub struct CoefficientSpectrum {
    c: Box<[Float]>,
}

impl CoefficientSpectrum {
    pub fn new(v: Float, n: usize) -> Self {
        Self {
            c: vec![v; n].into_boxed_slice(),
        }
    }
    pub fn is_black(&self) -> bool {
        for i in 0..self.c.len() {
            if self.c[i] != 0. {
                return false;
            }
        }
        true
    }
    pub fn sqrt(mut self) -> Self {
        for i in 0..self.c.len() {
            self.c[i] = self.c[i].sqrt();
        }
        self
    }
    pub fn lerp(self, t: Float, max: Self) -> Self {
        self * t + max * (1. - t)
    }
    pub fn clamp(self, min: Float, max: Float) -> Self {
        self.map(|x| clamp(x, min, max))
    }
    pub fn map<F: Fn(Float) -> Float>(mut self, f: F) -> Self {
        for i in 0..self.c.len() {
            self.c[i] = f(self.c[i]);
        }
        self
    }
    pub fn op<F: Fn(Float, Float) -> Float>(mut self, rhs: &Self, f: F) -> Self {
        for i in 0..self.c.len() {
            self.c[i] = f(self.c[i], rhs.c[i]);
        }
        self
    }
    pub fn has_nans(&self) -> bool {
        for i in 0..self.c.len() {
            if self.c[i].is_nan() {
                return true;
            }
        }
        false
    }
}

impl Index<usize> for CoefficientSpectrum {
    type Output = Float;
    fn index(&self, index: usize) -> &Self::Output {
        &self.c[index]
    }
}

impl_op_commutative!(+ |a: CoefficientSpectrum, b: &CoefficientSpectrum| -> CoefficientSpectrum {
    a.op(&b, |x, y| x + y)
});
impl_op!(+ |a: CoefficientSpectrum, b: CoefficientSpectrum| -> CoefficientSpectrum {
    a + &b
});
impl_op_commutative!(
    -|a: CoefficientSpectrum, b: &CoefficientSpectrum| -> CoefficientSpectrum {
        a.op(&b, |x, y| x - y)
    }
);
impl_op!(-|a: CoefficientSpectrum, b: CoefficientSpectrum| -> CoefficientSpectrum { a - &b });
impl_op_commutative!(
    *|a: CoefficientSpectrum, b: &CoefficientSpectrum| -> CoefficientSpectrum {
        a.op(&b, |x, y| x * y)
    }
);
impl_op!(*|a: CoefficientSpectrum, b: CoefficientSpectrum| -> CoefficientSpectrum { a * &b });
impl_op_commutative!(/ |a: CoefficientSpectrum, b: &CoefficientSpectrum| -> CoefficientSpectrum {
    a.op(&b, |x, y| x / y)
});
impl_op!(/ |a: CoefficientSpectrum, b: CoefficientSpectrum| -> CoefficientSpectrum {
    a / &b
});

impl_op_commutative!(+ |a: CoefficientSpectrum, b: Float| -> CoefficientSpectrum {
    a.map(|x| x + b)
});
impl_op_commutative!(-|a: CoefficientSpectrum, b: Float| -> CoefficientSpectrum {
    a.map(|x| x - b)
});
impl_op_commutative!(*|a: CoefficientSpectrum, b: Float| -> CoefficientSpectrum {
    a.map(|x| x * b)
});
impl_op_commutative!(/ |a: CoefficientSpectrum, b: Float| -> CoefficientSpectrum {
    a.map(|x| x / b)
});
