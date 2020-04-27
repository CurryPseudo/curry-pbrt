use crate::*;
use alga::general::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use num_traits::FromPrimitive;

mod distribution;
pub use distribution::*;

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

pub fn inverse_gamma_correct(f: Float) -> Float {
    if f <= 0.04045 {
        f * 1. / 12.92
    } else {
        ((f + 0.055) / 1.05).powf(2.4)
    }
}

pub fn coordinate_system(z: &Vector3f) -> (Vector3f, Vector3f) {
    let x = if z.x.abs() > z.y.abs() {
        Vector3f::new(-z.z, 0., z.x) / (z.x * z.x + z.z * z.z).sqrt()
    } else {
        Vector3f::new(0., z.z, -z.y) / (z.y * z.y + z.z * z.z).sqrt()
    };
    (x, z.cross(&x))
}

pub fn gamma(n: Integer) -> Float {
    let n_machine_epsilon = n as Float * MACHINE_EPSILON;
    n_machine_epsilon / (1. - n_machine_epsilon)
}

pub fn sample_usize_remap(u: Float, len: usize) -> (usize, Float) {
    let f = u * len as Float;
    let trunc = f.trunc();
    let remap = f - trunc;
    (min(trunc as usize, len - 1), remap)
}

pub fn sample_distribution_1d_remap(
    u: Float,
    len: usize,
    f: &dyn Fn(usize) -> Float,
) -> (usize, Float, Float) {
    Distribution1D::new(f, len).sample_remap(u)
}
pub fn polar_point(r: Float, theta: Float) -> Point2f {
    r * Point2f::new(theta.cos(), theta.sin())
}
pub fn concentric_sample_disk(u: Point2f) -> Point2f {
    let u = 2. * u - Vector2f::new(1., 1.);
    if u.x == 0. || u.y == 0. {
        Point2f::new(0., 0.)
    } else {
        let (r, theta) = if u.x.abs() > u.y.abs() {
            (u.x, (PI / 4.) * (u.y / u.x))
        } else {
            (u.y, (PI / 2.) - (PI / 4.) * (u.x / u.y))
        };
        polar_point(r, theta)
    }
}
pub fn uniform_sample_hemisphere(u: Point2f) -> Vector3f {
    let z = 1. - 2. * u.x;
    let r = max(0., 1. - z * z).sqrt();
    let phi = 2. * PI * u.y;
    Vector3f::new(r * phi.cos(), r * phi.sin(), z)
}
pub fn cosine_sample_hemisphere(u: Point2f) -> (Vector3f, Float) {
    let d = concentric_sample_disk(u);
    let z = (1. - d.coords.magnitude_squared()).sqrt();
    (Vector3f::new(d.x, d.y, z), z * INV_PI)
}

pub fn uniform_sample_triangle(u: Point2f) -> Point2f {
    let su0 = u.x.sqrt();
    Point2f::new(1. - su0, u.y * su0)
}

pub fn has_nan(p: &Point3f) -> bool {
    if p.x.is_nan() || p.y.is_nan() || p.z.is_nan() {
        return true;
    }
    false
}

pub fn spherical_to_normalize_phi_theta(w: &Vector3f) -> Point2f {
    let p = w.y.atan2(w.x);
    Point2f::new(
        if p < 0. { p + 2. * PI } else { p } / 2. * INV_PI,
        clamp(w.z, -1., 1.).acos() * INV_PI,
    )
}

pub fn normalize_phi_theta_to_spherical(phi_theta: &Point2f) -> Vector3f {
    let theta = phi_theta.y * PI;
    let phi = phi_theta.x * PI * 2.;
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();
    Vector3f::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
}
pub fn cos_theta(w: &Vector3f) -> Float {
    w.z
}
pub fn cos_2_theta(w: &Vector3f) -> Float {
    w.z * w.z
}
pub fn sin_2_theta(w: &Vector3f) -> Float {
    max(1. - cos_2_theta(w), 0.)
}
pub fn sin_theta(w: &Vector3f) -> Float {
    sin_2_theta(w).sqrt()
}
pub fn tan_theta(w: &Vector3f) -> Float {
    sin_theta(w) / cos_theta(w)
}
pub fn tan_2_theta(w: &Vector3f) -> Float {
    sin_2_theta(w) / cos_2_theta(w)
}
pub fn cos_phi(w: &Vector3f) -> Float {
    let sin_theta = sin_theta(w);
    if sin_theta == 0. {
        1.
    } else {
        clamp(w.x / sin_theta, -1., 1.)
    }
}
pub fn sin_phi(w: &Vector3f) -> Float {
    let sin_theta = sin_theta(w);
    if sin_theta == 0. {
        0.
    } else {
        clamp(w.y / sin_theta, -1., 1.)
    }
}
pub fn cos_2_phi(w: &Vector3f) -> Float {
    let cos_phi = cos_phi(w);
    cos_phi * cos_phi
}
pub fn sin_2_phi(w: &Vector3f) -> Float {
    let sin_phi = sin_phi(w);
    sin_phi * sin_phi
}
pub fn cos_delta_phi(wa: &Vector3f, wb: &Vector3f) -> Float {
    clamp(
        (wa.x * wb.x + wa.y * wb.y)
            / ((wa.x * wa.x + wa.y * wa.y) * (wb.x * wb.x + wb.y * wb.y)).sqrt(),
        -1.,
        1.,
    )
}
pub fn refract(wo: &Vector3f, n: &Normal3f, eta: Float) -> Option<Vector3f> {
    let cos_theta_o = wo.dot(n);
    let sin_2_theta_o = 1. - cos_theta_o * cos_theta_o;
    let sin_2_theta_i = sin_2_theta_o * eta * eta;
    if sin_2_theta_i > 1. {
        return None;
    }
    let cos_theta_i = max(1. - sin_2_theta_i, 0.).sqrt();
    Some(eta * (-wo) + (eta * cos_theta_o - cos_theta_i) * n.as_ref())
}
#[allow(clippy::excessive_precision)]
pub const INV_PI: Float = 0.31830988618379067154;

#[allow(clippy::excessive_precision)]
pub const PI: Float = 3.14159265358979323846;
