use crate::*;
use alga::general::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use num_traits::FromPrimitive;

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

pub fn coordinate_system(z: &Vector3f) -> (Vector3f, Vector3f) {
    let x = if z.x.abs() > z.y.abs() {
        Vector3f::new(-z.z, 0., z.x) / (z.x * z.x + z.z * z.z).sqrt()
    }
    else {
        Vector3f::new(0., z.z, -z.y) / (z.y * z.y + z.z * z.z).sqrt()
    };
    (x, z.cross(&x))
}

pub fn gamma(n: Integer) -> Float {
    let n_machine_epsilon = n as Float * MACHINE_EPSILON;
    n_machine_epsilon / (1. - n_machine_epsilon)
}
pub const INV_PI: Float = 0.31830988618379067154;

pub const PI: Float = 3.14159265358979323846;
