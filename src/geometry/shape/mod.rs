use super::{Bounds3f, Normal3f, Ray, RayIntersectCache, Vector3f, Point3f};
mod sphere;

pub trait Shape {
    fn bound(&self) -> Bounds3f;
    fn intersect(&self, ray: &Ray) -> Option<Interaction>;
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
    fn intersect_predicate_through_bound(&self, ray: &RayIntersectCache) -> bool {
        if self.bound().intersect_predicate_cached(ray) {
            self.intersect_predicate(ray.origin_ray())
        } else {
            false
        }
    }
}

pub struct Interaction {
    p: Point3f,
    n: Normal3f,
}
impl Interaction {

    pub fn new(p: Point3f, n: Normal3f) -> Self {
        Self { p, n }
    }
}
