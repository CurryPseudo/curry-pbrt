use crate::geometry::point::Point3f;
use crate::geometry::vector::Vector3f;
use crate::def::Float;

pub struct Ray {
    o: Point3f,
    d: Vector3f,
    t_max: Float   
}

impl Ray {
    pub fn new(o: Point3f, d: Vector3f, t_max: Float) -> Self {
        Self {o, d, t_max}
    }
}

pub struct RayDiffrential {
    r: Ray,
    rx: Ray,
    ry: Ray
}
