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
    fn visibility_test_ray(&self, _: &Point3f, _: &Vector3f) -> Ray {
        unreachable!()
    }
    fn sample_li(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
        scene: &Scene,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        let (shape_point, pdf) = self.shape.sample_by_point_pdf(point, sampler);
        let wi = (shape_point.p - point).normalize();
        let mut ray = Ray::new_od(*point, wi);
        ray.move_a_bit();
        let intersect = scene.intersect(&ray);
        if let Some(intersect) = intersect {
            if Arc::ptr_eq(&self.shape, intersect.get_shape()) {
                return (wi, Some(self.le(&shape_point)), pdf);
            }
        }
        (wi, None, pdf)
    }
    fn sample_li_without_visibility_test(
        &self,
        _: &Point3f,
        _: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        unreachable!()
    }
    fn box_apply(&self, _: &Transform) -> Box<dyn Light> {
        unreachable!()
    }
    fn le_pdf(&self, ray: &Ray, scene: &Scene) -> (Option<Spectrum>, Float) {
        if let Some(intersect) = self.shape.intersect(&ray) {
            let point = ray.o;
            let shape_point = intersect.get_shape_point();
            let pdf = self.shape.by_point_pdf(&point, shape_point);
            if let Some(intersect) = scene.intersect(&ray) {
                if Arc::ptr_eq(&self.shape, intersect.get_shape()) {
                    return (Some(self.le(shape_point)), pdf);
                }
            }
            (None, pdf)
        } else {
            (None, 0.)
        }
    }
    fn le(&self, _: &ShapePoint) -> Spectrum {
        self.le / self.shape.area()
    }
    fn le_out_scene(&self, _: &Ray) -> Float {
        0.
    }
    fn is_delta(&self) -> bool {
        false
    }
}
