use crate::*;
pub struct LensCamera<T> {
    camera: T,
    lens_radius: Float,
    focal_distance: Float,
}

impl<T> LensCamera<T> {
    pub fn new(camera: T, lens_radius: Float, focal_distance: Float) -> Self {
        Self{camera, lens_radius, focal_distance}
    }
}

impl<T: PrimitiveClipper> PrimitiveClipper for LensCamera<T> {
    fn clip(&self, primitive: &primitive::Primitive) -> bool {
        self.camera.clip(primitive)
    }
}

impl<T: Send + Sync + Camera> Camera for LensCamera<T> {
    fn as_clipper(&self) -> &dyn PrimitiveClipper {
        self
    }
    fn generate_ray(&self, film: Point2f, sampler: &mut dyn sampler::Sampler) -> Ray {
        let ray = self.camera.generate_ray(film, sampler);
        let lens = self.lens_radius * concentric_sample_disk(sampler.get_2d());
        let ft = self.focal_distance / ray.d.z;
        assert!(!ft.is_nan() && !ft.is_infinite());
        let focus = ray.eval(ft);
        let o = Point3f::new(lens.x, lens.y, 0.);
        let d = (focus - o).normalize();
        Ray::new_od(o, d)
    }
}

pub trait ParseWithLens<T> {
    fn with_lens(self, property_set: &PropertySet) -> Box<dyn Camera>;
}

impl<T: 'static + Camera> ParseWithLens<T> for T {
    fn with_lens(self, property_set: &PropertySet) -> Box<dyn Camera> {
        let lens_radius = property_set.get_value("lensradius");
        let focal_distance = property_set.get_value("focaldistance").unwrap_or(1e6);
        if let Some(lens_radius) = lens_radius {
            Box::new(LensCamera::new(self, lens_radius, focal_distance))
        }
        else {
            Box::new(self)
        }
    }
}
