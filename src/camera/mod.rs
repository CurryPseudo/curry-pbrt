mod perspective;
use crate::geometry::{Ray, Point2f};

pub trait Camera {
    fn generate_ray(&self, film: Point2f) -> Ray;
}
