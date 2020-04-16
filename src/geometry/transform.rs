use super::Vector3f;
use crate::def::Float;
use crate::scene_file_parser::BlockSegment;
use nalgebra::{geometry::Translation, Matrix4};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Transform {
    pub m: Matrix4<Float>,
    pub m_inv: Matrix4<Float>,
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.m)
    }
}

impl Transform {
    pub fn new(m: Matrix4<Float>, m_inv: Matrix4<Float>) -> Self {
        Self { m, m_inv }
    }
    pub fn inverse(self) -> Self {
        Self::new(self.m_inv, self.m)
    }
    pub fn transpose(self) -> Self {
        Self::new(self.m.transpose(), self.m_inv.transpose())
    }
    pub fn translate(delta: Vector3f) -> Self {
        Self::new(
            Matrix4::new(
                1., 0., 0., delta.x, 0., 1., 0., delta.y, 0., 0., 1., delta.z, 0., 0., 0., 1.,
            ),
            Matrix4::new(
                1., 0., 0., -delta.x, 0., 1., 0., -delta.y, 0., 0., 1., -delta.z, 0., 0., 0., 1.,
            ),
        )
    }
    pub fn scale(scale: Vector3f) -> Self {
        Self::new(
            Matrix4::new(
                scale.x, 0., 0., 0., 0., scale.y, 0., 0., 0., 0., scale.z, 0., 0., 0., 0., 1.,
            ),
            Matrix4::new(
                1. / scale.x,
                0.,
                0.,
                0.,
                0.,
                1. / scale.y,
                0.,
                0.,
                0.,
                0.,
                1. / scale.z,
                0.,
                0.,
                0.,
                0.,
                1.,
            ),
        )
    }
    pub fn has_scale(&self) -> bool {
        !is_norm(Vector3f::new(1., 0., 0.).apply(&self))
            || !is_norm(Vector3f::new(0., 1., 0.).apply(&self))
            || !is_norm(Vector3f::new(0., 0., 1.).apply(&self))
    }
    pub fn perspective(fov: Float, near: Float, far: Float) -> Self {
        let inv_tan_half_fov = 1. / (fov.to_radians() / 2.).tan();
        let t = far / (far - near);
        Self::from(Matrix4::new(
            inv_tan_half_fov,
            0.,
            0.,
            0.,
            0.,
            inv_tan_half_fov,
            0.,
            0.,
            0.,
            0.,
            t,
            -t * near,
            0.,
            0.,
            1.,
            0.,
        ))
    }
}
fn is_norm(v: Vector3f) -> bool {
    let l = v.magnitude();
    l <= 1.001 && l >= 0.999
}

impl From<Matrix4<Float>> for Transform {
    fn from(m: Matrix4<Float>) -> Self {
        Transform::new(m, m.try_inverse().unwrap())
    }
}

impl Default for Transform {
    fn default() -> Self {
        let identity = Matrix4::identity();
        Self {
            m: identity,
            m_inv: identity,
        }
    }
}

pub trait Transformable {
    fn apply(self, transform: &Transform) -> Self;
}

impl Transformable for Transform {
    fn apply(self, transform: &Transform) -> Self {
        Self::new(transform.m * self.m, self.m_inv * transform.m_inv)
    }
}

pub fn parse_transform(segment: &BlockSegment) -> Option<Transform> {
    let (transform_type, property_set) = segment.get_object().unwrap();
    match transform_type {
        "Translate" => Some(Transform::translate(Vector3f::new(
            property_set[0].get_float().unwrap(),
            property_set[1].get_float().unwrap(),
            property_set[2].get_float().unwrap(),
        ))),
        _ => None,
    }
}
