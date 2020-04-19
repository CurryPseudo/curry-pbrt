use super::{Transform, Transformable};
use crate::*;
use num_traits::Bounded;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_max: Float,
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "o: {} d: {} t_max: {}", self.o, self.d, self.t_max)
    }
}

impl Ray {
    pub fn new_od(o: Point3f, d: Vector3f) -> Self {
        Self {
            o,
            d,
            t_max: Float::max_value(),
        }
    }
    pub fn from_to(from: Point3f, to: Point3f) -> Self {
        let o = from;
        let d = to - from;
        let t_max = 0.999999999;
        Self {
            o,
            d,
            t_max,
        }
    }
    pub fn new(o: Point3f, d: Vector3f, t_max: Float) -> Self {
        Self { o, d, t_max }
    }
    pub fn eval(&self, t: Float) -> Point3f {
        &self.o + &self.d * t
    }
}

impl Transformable for Ray {
    fn apply(mut self, transform: &Transform) -> Self {
        self.o = self.o.apply(transform);
        self.d = self.d.apply(transform);
        self
    }
}
