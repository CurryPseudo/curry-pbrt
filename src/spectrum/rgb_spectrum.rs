use crate::{
    def::Float,
    scene_file_parser::{BasicTypes, ParseFromBasicType},
};
use derive_more::{Index, Into};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Index, Into)]
pub struct RGBSpectrum([Float; 3]);

impl RGBSpectrum {
    pub fn new(v: Float) -> Self {
        Self([v; 3])
    }
    pub fn map_move<F: Fn(Float) -> Float>(self, f: F) -> Self {
        Self([f(self.0[0]), f(self.0[1]), f(self.0[2])])
    }
    pub fn map<F: Fn(&mut Float)>(&mut self, f: F) {
        f(&mut self.0[0]);
        f(&mut self.0[1]);
        f(&mut self.0[2]);
    }
    pub fn op_move<F: Fn(Float, Float) -> Float>(self, rhs: Self, f: F) -> Self {
        Self([
            f(self.0[0], rhs.0[0]),
            f(self.0[1], rhs.0[1]),
            f(self.0[2], rhs.0[2]),
        ])
    }
    pub fn op<F: Fn(&mut Float, Float)>(&mut self, rhs: Self, f: F) {
        f(&mut self.0[0], rhs.0[0]);
        f(&mut self.0[1], rhs.0[1]);
        f(&mut self.0[2], rhs.0[2]);
    }
}

impl From<[Float; 3]> for RGBSpectrum {
    fn from(rgb: [Float; 3]) -> Self {
        Self(rgb)
    }
}
macro_rules! impl_num_op {
    ($op_trait:tt, $fn_name:ident, $op:tt) => {
        impl $op_trait<Self> for RGBSpectrum {
            type Output = Self;
            fn $fn_name(self, rhs: Self) -> Self::Output {
                self.op_move(rhs, |x, y| x $op y)
            }
        }
        impl $op_trait<&Self> for RGBSpectrum {
            type Output = Self;
            fn $fn_name(self, rhs: &Self) -> Self::Output {
                self.op_move(*rhs, |x, y| x $op y)
            }
        }
        impl $op_trait<Float> for RGBSpectrum {
            type Output = Self;
            fn $fn_name(self, rhs: Float) -> Self::Output {
                self.map_move(|x| x $op rhs)
            }
        }
        impl $op_trait<Option<Self>> for RGBSpectrum {
            type Output = Self;
            fn $fn_name(self, _rhs: Option<Self>) -> Self::Output {
                self.map_move(|x| x $op 0.)
            }
        }
        impl $op_trait<&Option<Self>> for RGBSpectrum {
            type Output = Self;
            fn $fn_name(self, _rhs: &Option<Self>) -> Self::Output {
                self.map_move(|x| x $op 0.)
            }
        }
    }
}
macro_rules! impl_num_op_assign {
    ($op_trait:tt, $fn_name:ident, $op:tt) => {
        impl $op_trait<Self> for RGBSpectrum {
            fn $fn_name(&mut self, rhs: Self) {
                self.op(rhs, |x, y| *x $op y);
            }
        }
        impl $op_trait<&Self> for RGBSpectrum {
            fn $fn_name(&mut self, rhs: &Self) {
                self.op(*rhs, |x, y| *x $op y);
            }
        }
        impl $op_trait<Float> for RGBSpectrum {
            fn $fn_name(&mut self, rhs: Float) {
                self.map(|x| *x $op rhs)
            }
        }
        impl $op_trait<Option<Self>> for RGBSpectrum {
            fn $fn_name(&mut self, _rhs: Option<Self>) {
                self.map(|x| *x $op 0.)
            }
        }
        impl $op_trait<&Option<Self>> for RGBSpectrum {
            fn $fn_name(&mut self, _rhs: &Option<Self>) {
                self.map(|x| *x $op 0.)
            }
        }
    }
}
impl_num_op!(Add, add, +);
impl_num_op!(Mul, mul, *);
impl_num_op!(Div, div, /);
impl_num_op!(Sub, sub, -);
impl_num_op_assign!(AddAssign, add_assign, +=);
impl_num_op_assign!(SubAssign, sub_assign, -=);
impl_num_op_assign!(MulAssign, mul_assign, *=);
impl_num_op_assign!(DivAssign, div_assign, /=);

impl ParseFromBasicType for RGBSpectrum {
    fn parse_from_basic_type(basic_type: &BasicTypes) -> Self {
        let floats = basic_type.get_floats().unwrap();
        if floats.len() != 3 {
            panic!()
        }
        RGBSpectrum::from([floats[0], floats[1], floats[2]])
    }
}
