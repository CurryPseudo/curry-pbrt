mod perspective;
use crate::*;
use downcast_rs::DowncastSync;
pub use perspective::*;

pub trait Camera: DowncastSync + PrimitiveClipper {
    fn generate_ray(&self, film: Point2f, sampler: &mut dyn Sampler) -> Ray;
    fn as_clipper(&self) -> &dyn PrimitiveClipper;
}

impl_downcast!(sync Camera);

impl ParseFromBlockSegment<'_> for Box<dyn Camera> {
    type T = Box<dyn Fn(Vector2u) -> Box<dyn Camera>>;
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T> {
        let object_value = segment.get_object_by_type("Camera")?;
        if object_value.get_name().unwrap() == "perspective" {
            let fov = object_value.get_value("fov").unwrap_or(90.);
            Some(Box::new(move |resolution| {
                Box::new(PerspectiveCamera::new(fov, resolution))
            }))
        } else {
            trace!("{:?}", segment);
            panic!()
        }
    }
}

pub struct TransformCamera {
    camera: Box<dyn Camera>,
    transform: Transform,
}

impl From<Box<dyn Camera>> for TransformCamera {
    fn from(camera: Box<dyn Camera>) -> Self {
        Self::new(camera, Transform::default())
    }
}
impl Transformable for TransformCamera {
    fn apply(self, transform: &Transform) -> Self {
        Self::new(
            self.camera,
            self.transform.apply(&transform.clone().inverse()),
        )
    }
}
impl TransformCamera {
    pub fn new(camera: Box<dyn Camera>, transform: Transform) -> Self {
        Self {
            camera,
            transform: transform.inverse(),
        }
    }
}

impl PrimitiveClipper for TransformCamera {
    fn clip(&self, primitive: &Primitive) -> bool {
        let inv = self.transform.clone().inverse();
        self.camera.clip(&primitive.clone().apply(&inv))
    }
}

impl Camera for TransformCamera {
    fn generate_ray(&self, film: Point2f, sampler: &mut dyn Sampler) -> Ray {
        self.camera.generate_ray(film, sampler).apply(&self.transform)
    }
    fn as_clipper(&self) -> &dyn PrimitiveClipper {
        self
    }
}

pub fn camera_apply(camera: Box<dyn Camera>, transform: &Transform) -> Box<dyn Camera> {
    match camera.downcast::<TransformCamera>() {
        Ok(transform_camera) => Box::new(transform_camera.apply(transform)),
        Err(camera) => Box::new(TransformCamera::new(camera, transform.clone())),
    }
}
