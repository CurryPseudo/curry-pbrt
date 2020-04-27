use super::DeltaLight;
use crate::*;

#[derive(Debug, Clone)]
pub struct PointLight {
    point: Point3f,
    i: Spectrum,
}

impl PointLight {
    pub fn new(i: Spectrum) -> Self {
        Self {
            point: Point3f::new(0., 0., 0.),
            i,
        }
    }
}

impl Transformable for PointLight {
    fn apply(self, transform: &Transform) -> Self {
        Self {
            point: self.point.apply(transform),
            i: self.i,
        }
    }
}

impl DeltaLight for PointLight {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray {
        let t = (self.point - point).magnitude();
        Ray::new(*point, *wi, t)
    }
    fn sample_li(&self, point: &ShapePoint) -> (Vector3f, Option<Spectrum>, VisibilityTester) {
        let wi = self.point - point.p;
        let i = self.i / wi.magnitude_squared();
        let to = ShapePoint::new_p_normal(self.point, Normal3f::from(-wi));
        (wi.normalize(), Some(i), VisibilityTester::new(point, &to))
    }
}
