use super::{Transform, Transformable, Vector3f};
use nalgebra::{Matrix3x4, Matrix4x3};
use std::{fmt::Display, ops::Deref};

#[derive(Debug, Clone, Copy)]
pub struct Normal3f(pub Vector3f);

impl From<Vector3f> for Normal3f {
    fn from(v: Vector3f) -> Self {
        Self(v.normalize())
    }
}
impl Into<Vector3f> for Normal3f {
    fn into(self) -> Vector3f {
        self.0
    }
}

impl AsRef<Vector3f> for Normal3f {
    fn as_ref(&self) -> &Vector3f {
        &self.0
    }
}

impl Deref for Normal3f {
    type Target = Vector3f;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Transformable for Normal3f {
    fn apply(self, transform: &Transform) -> Self {
        let m3 = Matrix3x4::identity() * (transform.m_inv.transpose() * Matrix4x3::identity());
        Self(m3 * self.0)
    }
}

impl Display for Normal3f {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
