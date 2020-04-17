use super::DeltaLight;
use crate::*;

#[derive(Clone)]
pub struct DistantLight {
    w: Vector3f,
    i: Spectrum,
}

impl DistantLight {
    pub fn new(w: Vector3f, i: Spectrum) -> Self {
        Self {
            w: w.normalize(),
            i,
        }
    }
}

impl Transformable for DistantLight {
    fn apply(self, transform: &Transform) -> Self {
        Self {
            w: self.w.apply(transform),
            i: self.i,
        }
    }
}

impl DeltaLight for DistantLight {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray {
        Ray::new_od(*point, *wi)
    }
    fn sample_li(&self, _point: &Point3f) -> (Vector3f, Option<Spectrum>) {
        (-self.w, Some(self.i))
    }
}
