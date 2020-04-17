mod area;
mod distant;
mod point;
use crate::{
    def::Float,
    geometry::{Point3f, Ray, Transform, Transformable, Vector3f, Shape, ShapePoint},
    sampler::Sampler,
    scene::Scene,
    scene_file_parser::PropertySet,
    spectrum::Spectrum,
};

pub use area::*;
pub use distant::*;
pub use point::*;
use std::sync::Arc;

pub trait Light: Sync + Send {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray;
    fn sample_li(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
        scene: &Scene,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        let li_pdf = self.sample_li_without_visibility_test(point, sampler);
        let (wi, li, pdf) = li_pdf;
        if li.is_some() {
            let mut ray = self.visibility_test_ray(&point, &wi);
            ray.move_a_bit();
            if scene.intersect_predicate(&ray) {
                (wi, None, pdf)
            } else {
                li_pdf
            }
        } else {
            li_pdf
        }
    }
    fn sample_li_without_visibility_test(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float);
    fn le_out_scene(&self, _: &Ray) -> Float {
        0.
    }
    fn le_pdf(&self, ray: &Ray, scene: &Scene) -> (Option<Spectrum>, Float);
    fn box_apply(&self, transform: &Transform) -> Box<dyn Light>;
    fn is_delta(&self) -> bool {
        false
    }
    fn le(&self, _: &ShapePoint) -> Spectrum {
        Spectrum::new(0.)
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
pub fn parse_area_light(property_set: &PropertySet) -> Box<dyn Fn(Arc<dyn Shape>) -> Box<dyn Light>> {
    match property_set.get_name().unwrap() {
        "diffuse" => {
            let l = property_set.get_default("L");
            Box::new(move |shape| Box::new(AreaLight::new(shape, l)))
        }
        _ => panic!(),
    }

}

pub trait DeltaLight: Clone + Transformable {
    fn sample_li(&self, point: &Point3f) -> (Vector3f, Option<Spectrum>);
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray;
}
impl<T: DeltaLight + 'static + Sync + Send> Light for T {
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray {
        self.visibility_test_ray(point, wi)
    }
    fn sample_li_without_visibility_test(
        &self,
        point: &Point3f,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float) {
        sampler.get_2d();
        let (wi, li) = self.sample_li(point);
        (wi, li, 1.)
    }
    fn box_apply(&self, transform: &Transform) -> Box<dyn Light> {
        Box::new(self.clone().apply(&transform))
    }
    fn is_delta(&self) -> bool {
        true
    }
    fn le_pdf(&self, _: &Ray, _: &Scene) -> (Option<Spectrum>, Float) {
        (None, 0.)
    }
}
