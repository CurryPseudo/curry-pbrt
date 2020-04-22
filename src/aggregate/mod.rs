use crate::*;
mod plain;
mod bvh;
pub use plain::*;
pub use bvh::*;
use std::mem;

pub trait Aggregate: Sync {
    fn build(&mut self, primitives: Vec<Primitive>);
    fn intersect_predicate(&self, ray: &Ray) -> bool;
    fn intersect(&self, ray: &Ray) -> Option<PrimitiveIntersect>;
}

pub enum AggregateBuilder {
    Builded(Box<dyn Aggregate>),
    ToBuild(Vec<Primitive>),
}

impl AggregateBuilder {
    pub fn get(&self) -> &dyn Aggregate {
        if let Self::Builded(aggregate) = self {
            aggregate.as_ref()
        } else {
            panic!("Primitives has not been builded to aggregate!");
        }
    }
    pub fn build(&mut self, mut aggregate: Box<dyn Aggregate>) {
        if let Self::ToBuild(primitives) = self {
            let primitives = mem::replace(primitives, Vec::new());
            aggregate.build(primitives);
            *self = Self::Builded(aggregate);
        } else {
            panic!("Primitives has been builded!");
        }
    }
    pub fn add_primitive(&mut self, primitive: Primitive) {
        if let Self::ToBuild(primitives) = self {
            primitives.push(primitive);
        } else {
            panic!("Primitives has been builded!");
        }
    }
}

impl Default for AggregateBuilder {
    fn default() -> Self {
        Self::ToBuild(Vec::new())
    }
}
