use alga::general::{ClosedAdd, ClosedMul, ClosedSub};
use num_traits::FromPrimitive;
use crate::def::Float;

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

pub static PI: Float = 3.14159265358979323846;
pub static INV_PI: Float = 0.31830988618379067154;
