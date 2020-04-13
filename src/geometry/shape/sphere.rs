use super::{Shape, ShapeIntersect};
use crate::{
    def::{Double, Float},
    geometry::{Bounds3f, Normal3f, Point3f, Ray, Vector3f, Point2f},
    math::{clamp, INV_PI, PI},
};
use std::mem::swap;

pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self { Self { radius } }
}


impl Shape for Sphere {
    fn bound(&self) -> Bounds3f {
        Bounds3f::new(
            Point3f::from(Vector3f::from_element(-self.radius)),
            Point3f::from(Vector3f::from_element(self.radius)),
        )
    }
    fn intersect(&self, ray: &Ray) -> Option<ShapeIntersect> {
        let a = ray.d.magnitude_squared();
        let b = 2. * ray.d.dot(&ray.o.coords);
        let c = ray.o.coords.magnitude_squared() - self.radius * self.radius;
        let (t0, t1) = solve_quadratic(a, b, c)?;
        if t0 > ray.t_max || t1 < 0. {
            None
        } else {
            let t = if t0 < 0. { t1 } else { t0 };
            let mut p = ray.eval(t);
            p *= self.radius / p.coords.magnitude();
            let n = Normal3f(p.coords.normalize());
            let u = (p.y.atan2(p.x) + PI) * 0.5 * INV_PI;
            let v = clamp(p.z / self.radius, -1., 1.).acos();
            Some(ShapeIntersect::new(p, n, t, Point2f::new(u, v)))
        }
    }
}

fn solve_quadratic(a: Float, b: Float, c: Float) -> Option<(Float, Float)> {
    let a = a as Double;
    let b = b as Double;
    let c = c as Double;
    let discrim = b * b - 4. * a * c;
    if discrim < 0. {
        None
    } else {
        let root_discrim = discrim.sqrt();
        let q = if b < 0. {
            -0.5 * (b - root_discrim)
        } else {
            -0.5 * (b + root_discrim)
        };
        let mut t0 = q / a;
        let mut t1 = c / q;
        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }
        Some((t0 as Float, t1 as Float))
    }
}
