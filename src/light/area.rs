use super::Light;
use crate::*;
use std::sync::Arc;

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
    fn le_out_scene(&self, _: &Ray) -> Float {
        0.
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
        let wi = (light_point.p - point.p).normalize();
        (
            wi,
            self.le(&light_point),
            pdf,
            VisibilityTester::new(point, &light_point),
        )
    }
}
