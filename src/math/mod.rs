use crate::def::Float;
use alga::general::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use num_traits::FromPrimitive;

mod with_pdf;
pub use with_pdf::*;

pub fn lerp<T: FromPrimitive + ClosedMul + ClosedAdd + ClosedSub + Copy>(
    t: T,
    min: T,
    max: T,
) -> T {
    min * t + max * (T::from_i32(1).unwrap() - t)
}
pub fn rlerp<T: FromPrimitive + ClosedDiv + ClosedAdd + ClosedSub + Copy>(
    t: T,
    min: T,
    max: T,
) -> T {
    (t - min) / (max - min)
}
pub fn clamp<T: PartialOrd>(t: T, min: T, max: T) -> T {
    if t < min {
        min
    } else if t > max {
        max
    } else {
        t
    }
}

pub fn power_heuristic(f: Float, g: Float) -> Float {
    (f * f) / (f * f + g * g)
}

pub fn min<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if rhs < lhs {
        rhs
    } else {
        lhs
    }
}
pub fn max<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if rhs > lhs {
        rhs
    } else {
        lhs
    }
}

pub fn gamma_correct(f: Float) -> Float {
    if f <= 0.0031308 {
        12.92 * f
    } else {
        1.055 * f.powf(1. / 2.4) - 0.055
    }
}
pub static INV_PI: Float = 0.31830988618379067154;

pub static PI: Float = 3.14159265358979323846;
