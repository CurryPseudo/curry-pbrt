mod perspective;
use crate::geometry::parse_transform;
use crate::geometry::{Point2f, Ray, Vector2u};
use crate::scene_file_parser::BlockSegment;
pub use perspective::*;
use std::collections::VecDeque;

pub trait Camera {
    fn generate_ray(&self, film: Point2f) -> Ray;
}
pub fn parse_camera(
    segment: &BlockSegment,
) -> Option<Box<dyn Fn(Vector2u) -> Box<dyn Camera>>> {
    let object_value = segment.get_object_by_type("Camera")?;
    if object_value.get_name().unwrap() == "perspective" {
        let fov = object_value.get_value("fov").unwrap_or(90.);
        Some(Box::new(move |resolution| {
            Box::new(PerspectiveCamera::new(fov, resolution))
        }))
    } else {
        panic!()
    }
}
