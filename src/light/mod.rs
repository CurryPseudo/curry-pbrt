mod point;
use crate::{
    def::Float,
    geometry::{Point3f, Ray, Transform, Vector3f},
    math::WithPdf,
    sampler::SamplerWrapper,
    scene_file_parser::PropertySet,
    spectrum::{parse_spectrum_default, Spectrum}, scene::Scene,
};

pub use point::*;

pub trait Light {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray;
    fn sample_li_with_visibility_test(&self, point: &Point3f, sampler: &mut SamplerWrapper, scene: &Scene) -> WithPdf<(Vector3f, Option<Spectrum>)> {
        let li_pdf = self.sample_li(point, sampler);
        let (wi, li) = li_pdf.t;
        if li.is_some() {
            let point = point + wi * 0.001;
            let ray = self.visibility_test_ray(&point, &wi);
            if scene.intersect_predicate(&ray) {
                WithPdf::new((wi, None), li_pdf.pdf)
            }
            else {
                li_pdf
            }
        }
        else {
            li_pdf
        }
    }
    fn sample_li(
        &self,
        point: &Point3f,
        sampler: &mut SamplerWrapper,
    ) -> WithPdf<(Vector3f, Option<Spectrum>)>;
    fn le(&self, ray: &Ray) -> Option<Spectrum>;
    fn pdf(&self, ray: &Ray) -> Float;
    fn le_pdf(&self, ray: &Ray) -> WithPdf<Option<Spectrum>> {
        WithPdf::new(self.le(ray), self.pdf(ray))
    }
    fn box_apply(&mut self, transform: &Transform) -> Box<dyn Light>;
}


pub fn parse_light(property_set: &PropertySet) -> Box<dyn Light> {
    match property_set.get_name().unwrap() {
        "point" => {
            let i = parse_spectrum_default(property_set, "I");
            Box::new(PointLight::new(i))
        }
        _ => panic!(),
    }
}
