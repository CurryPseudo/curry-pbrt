use super::Vector3f;

pub struct Normalf {
    internal: Vector3f,
}

impl From<Vector3f> for Normalf {
    fn from(internal: Vector3f) -> Self {
        Self { internal }
    }
}


