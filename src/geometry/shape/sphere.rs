use super::{Shape, ShapeIntersect, ShapePoint};
use crate::{
    def::{Double, Float},
    geometry::{uniform_sample_hemisphere, Bounds3f, Normal3f, Point2f, Point3f, Ray, Vector3f},
    math::{clamp, coordinate_system, INV_PI, PI},
    sampler::Sampler,
};
use std::mem::swap;

#[derive(Clone)]
pub struct Sphere {
    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
    fn calc_uv(&self, p: Point3f) -> Point2f {
        let u = (p.y.atan2(p.x) + PI) * 0.5 * INV_PI;
        let v = clamp(p.z / self.radius, -1., 1.).acos() * INV_PI;
        return Point2f::new(u, v);
    }
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
        let inside = c < 0.;
        let (t0, t1) = solve_quadratic(a, b, c)?;
        if t0 > ray.t_max || t1 < 0. {
            None
        } else {
            let t = if t0 < 0. { t1 } else { t0 };
            if t > ray.t_max {
                return None;
            }
            let mut p = ray.eval(t);
            p *= self.radius / p.coords.magnitude();
            let n = p.coords.normalize();
            let n = if inside { Normal3f(-n) } else { Normal3f(n) };
            Some(ShapeIntersect::new(p, n, t, self.calc_uv(p)))
        }
    }
    fn sample(&self, sampler: &mut dyn Sampler) -> ShapePoint {
        let d = uniform_sample_hemisphere(sampler.get_2d());
        let p = Point3f::from(d * self.radius);
        let n = Normal3f::from(d);
        ShapePoint::new(p, n, self.calc_uv(p))
    }
    fn sample_by_point(&self, point: &Point3f, sampler: &mut dyn Sampler) -> ShapePoint {
        let distance_2 = point.coords.magnitude_squared();
        let radius_2 = self.radius * self.radius;
        if distance_2 < radius_2 {
            return self.sample(sampler);
        }
        let distance = distance_2.sqrt();
        let z = point.coords / distance;
        let (x, y) = coordinate_system(&z);

        let u = sampler.get_2d();

        let sin_theta_max_2 = radius_2 / distance_2;
        let cos_theta_max = (1. - sin_theta_max_2).sqrt();
        let cos_theta = (1. - u.x) + u.x * cos_theta_max;
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let phi = u.y * 2. * PI;
        let ds = distance * cos_theta - (radius_2 - distance_2 * sin_theta * sin_theta).sqrt();
        let cos_alpha = (distance_2 + radius_2 - ds * ds) / (2. * distance * self.radius);
        let sin_alpha = (1. - cos_alpha * cos_alpha).sqrt();

        let d = cos_alpha * z + sin_alpha * phi.cos() * x + sin_alpha * phi.sin() * y;
        let n = Normal3f::from(d);
        let p = Point3f::from(d * self.radius);
        ShapePoint::new(p, n, self.calc_uv(p))
    }
    fn by_point_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        let distance_2 = point.coords.magnitude_squared();
        if distance_2 < self.radius * self.radius {
            self.default_by_point_pdf(point, shape_point)
        } else {
            let sin_theta_max_2 = self.radius * self.radius / distance_2;
            let cos_theta_max = (1. - sin_theta_max_2).sqrt();
            1. / (2. * PI * (1. - cos_theta_max))
        }
    }
    fn area(&self) -> Float {
        self.radius * self.radius * PI * 4.
    }
    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
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
