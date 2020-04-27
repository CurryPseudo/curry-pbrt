use super::Light;
use crate::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct AreaLight {
    shape: Arc<dyn Shape>,
    le: Spectrum,
}

impl AreaLight {
    pub fn new(shape: Arc<dyn Shape>, le: Spectrum) -> Self {
        Self { shape, le }
    }
}

impl Light for AreaLight {
    fn box_apply(&self, _: &Transform) -> Box<dyn Light> {
        unreachable!()
    }
    fn le(&self, _: &ShapePoint) -> Option<Spectrum> {
        Some(self.le)
    }
    fn is_delta(&self) -> bool {
        false
    }
    fn pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float {
        self.shape.by_point_pdf(point, shape_point)
    }
    fn sample_li(
        &self,
        point: &ShapePoint,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float, VisibilityTester) {
        let (light_point, pdf) = self.shape.sample_by_point(&point.p, sampler);
        if (light_point.p - point.p).magnitude_squared() == 0. || pdf == 0. {
            return (
                Vector3f::new(0., 0., 0.),
                None,
                0.,
                VisibilityTester::new(point, &light_point),
            );
        }
        let wi = (light_point.p - point.p).normalize();
        (
            wi,
            self.le(&light_point),
            pdf,
            VisibilityTester::new(point, &light_point),
        )
    }
}
