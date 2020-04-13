use crate::def::Float;
use alga::general::{ClosedAdd, ClosedMul, ClosedSub};
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

pub static INV_PI: Float = 0.31830988618379067154;

pub static PI: Float = 3.14159265358979323846;
