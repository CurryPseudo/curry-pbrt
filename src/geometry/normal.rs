use super::{Transform, Transformable, Vector3f};
use nalgebra::{Matrix3x4, Matrix4x3};

#[derive(Debug)]
pub struct Normal3f(pub Vector3f);

impl From<Vector3f> for Normal3f {
    fn from(v: Vector3f) -> Self {
        Self(v)
    }
}

impl Transformable for Normal3f {
    fn apply(self, transform: &Transform) -> Self {
        let m3 = Matrix3x4::identity() * (transform.m_inv.transpose() * Matrix4x3::identity());
        Self(m3 * self.0)
    }
}
