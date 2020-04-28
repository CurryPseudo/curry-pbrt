use crate::*;
use nalgebra::Matrix4;
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
    pub fn look_at(pos: Point3f, look: Point3f, up: Vector3f) -> Self {
        let dir = (look - pos).normalize();
        let right = up.normalize().cross(&dir).normalize();
        let new_up = dir.cross(&right);
        let m_inv = Matrix4::new(
            right.x, new_up.x, dir.x, pos.x, right.y, new_up.y, dir.y, pos.y, right.z, new_up.z,
            dir.z, pos.z, 0., 0., 0., 1.,
        );
        Self::new(m_inv.try_inverse().unwrap(), m_inv)
    }
    pub fn rotate(angle: Float, axis: Vector3f) -> Self {
        let axis = axis.normalize();
        let radians = angle.to_radians();
        let sin_value = radians.sin();
        let cos_value = radians.cos();
        let m = Matrix4::new(
            axis.x * axis.x + (1. - axis.x * axis.x) * cos_value,
            axis.x * axis.y * (1. - cos_value) + axis.z * sin_value,
            axis.x * axis.z * (1. - cos_value) - axis.y * sin_value,
            0.,
            axis.x * axis.y * (1. - cos_value) - axis.z * sin_value,
            axis.y * axis.y + (1. - axis.y * axis.y) * cos_value,
            axis.y * axis.z * (1. - cos_value) + axis.x * sin_value,
            0.,
            axis.x * axis.z * (1. - cos_value) + axis.y * sin_value,
            axis.y * axis.z * (1. - cos_value) - axis.x * sin_value,
            axis.z * axis.z + (1. - axis.z * axis.z) * cos_value,
            0.,
            0.,
            0.,
            0.,
            1.,
        );
        Self {
            m,
            m_inv: m.transpose(),
        }
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

impl ParseFromProperty for Transform {
    fn parse_from_property(type_name: &str, basic_types: &BasicTypes) -> Self {
        let fs = Vec::parse_from_property(type_name, basic_types);
        let m = Matrix4::from_vec(fs);
        Self::from(m)
    }
    fn parse_default() -> Self {
        Transform::default()
    }
}

impl ParseConsumeProperty for Transform {
    fn consume_size() -> usize {
        16
    }
}

impl ParseFromBlockSegment<'_> for Transform {
    type T = Transform;
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T> {
        let (transform_type, property_set) = segment.get_object()?;
        let mut property_set = property_set.clone();
        match transform_type {
            "ConcatTransform" => Some(property_set.get_no_type_value().unwrap()),
            "Transform" => Some(property_set.get_no_type_value().unwrap()),
            "Translate" => Some(Transform::translate(
                property_set.get_no_type_value().unwrap(),
            )),
            "Rotate" => Some(Transform::rotate(
                property_set.get_no_type_value().unwrap(),
                property_set.get_no_type_value().unwrap(),
            )),
            "Scale" => Some(Transform::scale(property_set.get_no_type_value().unwrap())),
            "LookAt" => Some(Transform::look_at(
                property_set.get_no_type_value().unwrap(),
                property_set.get_no_type_value().unwrap(),
                property_set.get_no_type_value().unwrap(),
            )),
            _ => None,
        }
    }
}
