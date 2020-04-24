mod area;
mod distant;
mod point;
use crate::*;

pub use area::*;
pub use distant::*;
pub use point::*;
use std::sync::Arc;

pub trait Light: Sync + Send {
    fn sample_li(
        &self,
        point: &ShapePoint,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float, VisibilityTester);
    fn le_out_scene(&self, _: &Ray) -> Option<Spectrum> {
        None
    }
    fn pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> Float;
    fn box_apply(&self, transform: &Transform) -> Box<dyn Light>;
    fn is_delta(&self) -> bool {
        false
    }
    fn le(&self, _: &ShapePoint) -> Option<Spectrum> {
        None
    }
    fn le_pdf(&self, point: &Point3f, shape_point: &ShapePoint) -> (Option<Spectrum>, Float) {
        (self.le(shape_point), self.pdf(point, shape_point))
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
            let w = property_set
                .get_value::<Point3f>("from")
                .map_or(Vector3f::new(0., 0., -1.), |from| {
                    property_set.get_value::<Point3f>("to").unwrap() - from
                });
            Box::new(DistantLight::new(w, i))
        }
        _ => panic!(),
    }
}
pub type AreaLightFactory = Arc<dyn Fn(Arc<dyn Shape>) -> Box<dyn Light>>;
pub fn parse_area_light(
    property_set: &PropertySet,
) -> AreaLightFactory {
    match property_set.get_name().unwrap() {
        "diffuse" => {
            let l = property_set.get_default("L");
            Arc::new(move |shape| Box::new(AreaLight::new(shape, l)))
        }
        _ => panic!(),
    }
}

pub trait DeltaLight: Clone + Transformable {
    fn sample_li(&self, point: &ShapePoint) -> (Vector3f, Option<Spectrum>, VisibilityTester);
    fn visibility_test_ray(&self, point: &Point3f, wi: &Vector3f) -> Ray;
}
impl<T: DeltaLight + 'static + Sync + Send> Light for T {
    fn box_apply(&self, transform: &Transform) -> Box<dyn Light> {
        Box::new(self.clone().apply(&transform))
    }
    fn is_delta(&self) -> bool {
        true
    }
    fn pdf(&self, _: &Point3f, _: &ShapePoint) -> Float {
        0.
    }
    fn sample_li(
        &self,
        point: &ShapePoint,
        sampler: &mut dyn Sampler,
    ) -> (Vector3f, Option<Spectrum>, Float, VisibilityTester) {
        sampler.get_2d();
        let (wi, s, visibility_tester) = self.sample_li(point);
        (wi, s, 1., visibility_tester)
    }
}

pub struct VisibilityTester(Ray);
impl VisibilityTester {
    pub fn new(from: &ShapePoint, to: &ShapePoint) -> Self {
        let from = from.point_offset_by_error(&(to.p - from.p));
        let to = to.point_offset_by_error(&(from - to.p));
        Self(Ray::from_to(from, to))
    }
    pub fn new_od(o: &ShapePoint, d: &Vector3f) -> Self {
        Self(Ray::new_od(o.point_offset_by_error(d), *d))
    }

    pub fn unoccluded(&self, scene: &Scene) -> bool {
        !scene.intersect_predicate(&self.0)
    }
}
