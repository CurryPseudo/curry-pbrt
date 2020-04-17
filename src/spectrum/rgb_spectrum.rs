use crate::*;
use derive_more::{Deref, Index, Into};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, Index, Into, Deref)]
pub struct RGBSpectrum([Float; 3]);

impl Display for RGBSpectrum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

impl Default for RGBSpectrum {
    fn default() -> Self {
        Self::new(0.)
    }
}

impl From<Option<RGBSpectrum>> for RGBSpectrum {
    fn from(s: Option<RGBSpectrum>) -> Self {
        s.unwrap_or(Self::default())
    }
}

impl RGBSpectrum {
    pub fn new(v: Float) -> Self {
        Self([v; 3])
    }
    pub fn is_black(&self) -> bool {
        self.0[0] == 0. && self.0[1] == 0. && self.0[2] == 0.
    }
    pub fn to_option(self) -> Option<Self> {
        if self.is_black() {
            None
        } else {
            Some(self)
        }
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
    pub fn to_xyz(self) -> Self {
        let mut r = [0., 0., 0.];
        r[0] = 0.412453 * self.0[0] + 0.357580 * self.0[1] + 0.180423 * self.0[2];
        r[1] = 0.212671 * self.0[0] + 0.715160 * self.0[1] + 0.072169 * self.0[2];
        r[2] = 0.019334 * self.0[0] + 0.119193 * self.0[1] + 0.950227 * self.0[2];
        Self(r)
    }
    pub fn from_xyz(self) -> Self {
        let mut r = [0., 0., 0.];
        r[0] = 3.240479 * self.0[0] - 1.537150 * self.0[1] - 0.498535 * self.0[2];
        r[1] = -0.969256 * self.0[0] + 1.875991 * self.0[1] + 0.041556 * self.0[2];
        r[2] = 0.055648 * self.0[0] - 0.204043 * self.0[1] + 1.057311 * self.0[2];
        Self(r)
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
            fn $fn_name(self, rhs: Option<Self>) -> Self::Output {
                self.op_move(rhs.unwrap_or(Self::new(0.)), |x, y| x $op y)
            }
        }
        impl $op_trait<&Option<Self>> for RGBSpectrum {
            type Output = Self;
            fn $fn_name(self, rhs: &Option<Self>) -> Self::Output {
                self.op_move(rhs.unwrap_or(Self::new(0.)), |x, y| x $op y)
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
            fn $fn_name(&mut self, rhs: Option<Self>) {
                self.op(rhs.unwrap_or(Self::new(0.)), |x, y| *x $op y)
            }
        }
        impl $op_trait<&Option<Self>> for RGBSpectrum {
            fn $fn_name(&mut self, rhs: &Option<Self>) {
                self.op(rhs.unwrap_or(Self::new(0.)), |x, y| *x $op y)
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

impl ParseFromProperty for RGBSpectrum {
    fn parse_from_property(_: &str, basic_type: &BasicTypes) -> Self {
        let floats = basic_type.get_floats().unwrap();
        if floats.len() != 3 {
            panic!()
        }
        RGBSpectrum::from([floats[0], floats[1], floats[2]])
    }
    fn parse_default() -> Self {
        RGBSpectrum::new(1.)
    }
}
