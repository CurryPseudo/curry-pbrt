use super::Vector3f;
use crate::def::Float;
use nalgebra::{geometry::Translation, Matrix4};

#[derive(Clone)]
pub struct Transform {
    pub m: Matrix4<Float>,
    pub m_inv: Matrix4<Float>,
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
            Matrix4::from([
                [1., 0., 0., delta.x],
                [0., 1., 0., delta.y],
                [0., 0., 1., delta.z],
                [0., 0., 0., 1.],
            ]),
            Matrix4::from([
                [1., 0., 0., -delta.x],
                [0., 1., 0., -delta.y],
                [0., 0., 1., -delta.z],
                [0., 0., 0., 1.],
            ]),
        )
    }
    pub fn scale(scale: Vector3f) -> Self {
        Self::new(
            Matrix4::from([
                [scale.x, 0., 0., 0.],
                [0., scale.y, 0., 0.],
                [0., 0., scale.z, 0.],
                [0., 0., 0., 1.],
            ]),
            Matrix4::from([
                [1. / scale.x, 0., 0., 0.],
                [0., 1. / scale.y, 0., 0.],
                [0., 0., 1. / scale.z, 0.],
                [0., 0., 0., 1.],
            ]),
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
        Self::from(Matrix4::from([
            [inv_tan_half_fov, 0., 0., 0.],
            [0., inv_tan_half_fov, 0., 0.],
            [0., 0., t, -t * near],
            [0., 0., 1., 0.],
        ]))
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
