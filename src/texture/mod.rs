use crate::geometry::Point2f;

pub struct Texture<T> {
    t: T,
}

impl<T> From<T> for Texture<T> {
    fn from(t: T) -> Self {
        Self { t }
    }
}

impl<T: Clone> Texture<T> {
    pub fn evaluate(&self, uv: &Point2f) -> T {
        self.t.clone()
    }
}
