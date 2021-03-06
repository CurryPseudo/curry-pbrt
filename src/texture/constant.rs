use crate::*;
#[derive(Debug)]
pub struct ConstantTexture<T: std::fmt::Debug>(T);

impl<T: std::fmt::Debug> From<T> for ConstantTexture<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T: Clone + Send + Sync + std::fmt::Debug> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _: &Point2f) -> T {
        self.0.clone()
    }
    fn pixels(&self) -> FixedVec2D<T> {
        FixedVec2D::new(self.0.clone(), Vector2u::new(1, 1))
    }
}
