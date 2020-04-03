use super::{Transform, Transformable, Vector3f};

pub struct Normalf(Vector3f);

impl From<Vector3f> for Normalf {
    fn from(v: Vector3f) -> Self {
        Self(v)
    }
}

impl Transformable for Normalf {
    fn apply(self, transform: &Transform) -> Self {
        Self(
            Vector3f::from_homogeneous(transform.m_inv.transpose() * self.0.to_homogeneous())
                .unwrap(),
        )
    }
}
