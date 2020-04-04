use super::{Transform, Transformable, Vector3f};

pub struct Normal3f(pub Vector3f);

impl From<Vector3f> for Normal3f {
    fn from(v: Vector3f) -> Self {
        Self(v)
    }
}

impl Transformable for Normal3f {
    fn apply(self, transform: &Transform) -> Self {
        Self(
            Vector3f::from_homogeneous(transform.m_inv.transpose() * self.0.to_homogeneous())
                .unwrap(),
        )
    }
}
