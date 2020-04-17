use super::DeltaLight;
use crate::*;

#[derive(Clone)]
pub struct DistantLight {
    from: Point3f,
    w: Vector3f,
    i: Spectrum,
}

impl DistantLight {
    pub fn new(from: Point3f, w: Vector3f, i: Spectrum) -> Self {
        Self {
            from,
            w: w.normalize(),
            i,
        }
    }
}

impl Transformable for DistantLight {
    fn apply(self, transform: &Transform) -> Self {
        Self {
            from: self.from,
            w: self.w.apply(transform),
            i: self.i,
        }
    }
}

impl DeltaLight for DistantLight {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray {
        let t = (self.from - point).dot(wi);
        Ray::new(*point, *wi, t)
    }
    fn sample_li(&self, point: &Point3f) -> (Vector3f, Option<Spectrum>) {
        let t = (self.from - point).dot(&-self.w);
        if t < 0. {
            (-self.w, None)
        } else {
            (-self.w, Some(self.i))
        }
    }
}
