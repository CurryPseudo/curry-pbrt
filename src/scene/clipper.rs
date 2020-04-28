use crate::*;
pub trait PrimitiveClipper {
    fn clip(&self, primitive: &Primitive) -> bool;
}
