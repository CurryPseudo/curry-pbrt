mod distant;
mod point;
use crate::{
    def::Float,
    geometry::{Point3f, Ray, Transform, Transformable, Vector3f},
    math::WithPdf,
    sampler::Sampler,
    scene::Scene,
    scene_file_parser::PropertySet,
    spectrum::Spectrum,
};

pub use distant::*;
pub use point::*;

pub trait Light: Sync {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray;
    fn sample_li_with_visibility_test(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
        scene: &Scene,
    ) -> WithPdf<(Vector3f, Option<Spectrum>)> {
        let li_pdf = self.sample_li(point, sampler);
        let (wi, li) = li_pdf.t;
        if li.is_some() {
            let point = point + wi * 0.001;
            let ray = self.visibility_test_ray(&point, &wi);
            if scene.intersect_predicate(&ray) {
                WithPdf::new((wi, None), li_pdf.pdf)
            } else {
                li_pdf
            }
        } else {
            li_pdf
        }
    }
    fn sample_li(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> WithPdf<(Vector3f, Option<Spectrum>)>;
    fn le(&self, ray: &Ray) -> Option<Spectrum>;
    fn pdf(&self, ray: &Ray) -> Float;
    fn le_pdf(&self, ray: &Ray) -> WithPdf<Option<Spectrum>> {
        WithPdf::new(self.le(ray), self.pdf(ray))
    }
    fn box_apply(&mut self, transform: &Transform) -> Box<dyn Light>;
    fn is_delta(&self) -> bool {
        false
    }
}

pub fn parse_light(property_set: &PropertySet) -> Box<dyn Light> {
    match property_set.get_name().unwrap() {
        "point" => {
            let i = property_set.get_default("I");
            Box::new(PointLight::new(i))
        }
        "distant" => {
            let i = property_set.get_default("L");
            let from = property_set
                .get_value("from")
                .unwrap_or(Point3f::new(0., 0., 0.));
            let w = property_set
                .get_value::<Point3f>("from")
                .map_or(Vector3f::new(0., 0., -1.), |from| {
                    property_set.get_value::<Point3f>("to").unwrap() - from
                });
            Box::new(DistantLight::new(from, w, i))
        }
        _ => panic!(),
    }
}

pub trait DeltaLight: Clone + Transformable {
    fn sample_li(&self, point: &Point3f) -> (Vector3f, Option<Spectrum>);
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray;
}
impl<T: DeltaLight + 'static + Sync> Light for T {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray {
        self.visibility_test_ray(point, wi)
    }
    fn sample_li(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> WithPdf<(Vector3f, Option<Spectrum>)> {
        sampler.get_2d();
        let (wi, li) = self.sample_li(point);
        WithPdf::new((wi, li), 1.)
    }
    fn le(&self, _: &Ray) -> Option<Spectrum> {
        None
    }
    fn pdf(&self, _: &Ray) -> Float {
        0.
    }
    fn box_apply(&mut self, transform: &Transform) -> Box<dyn Light> {
        Box::new(self.clone().apply(&transform))
    }
    fn is_delta(&self) -> bool {
        true
    }
}
