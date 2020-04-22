use crate::*;
#[derive(Default)]
pub struct PlainAggregate {
    primitives: Vec<Primitive>,
}

impl Aggregate for PlainAggregate {
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        let ray = RayIntersectCache::from(*ray);
        for primitive in &self.primitives {
            if primitive.intersect_predicate_through_bound(&ray) {
                return true;
            }
        }
        false
    }
    fn intersect(&self, ray: &Ray) -> Option<PrimitiveIntersect> {
        let mut intersect: Option<PrimitiveIntersect> = None;
        let mut ray = RayIntersectCache::from(*ray);
        for primitive in &self.primitives {
            let this_intersect = primitive.intersect_through_bound(&ray);
            if let Some(intersect) = &mut intersect {
                if let Some(this_intersect) = this_intersect {
                    ray.update_t_max(this_intersect.get_shape_intersect().get_t());
                    if this_intersect.get_shape_intersect().get_t()
                        < intersect.get_shape_intersect().get_t()
                    {
                        *intersect = this_intersect;
                    }
                }
            } else {
                intersect = this_intersect;
            }
        }
        intersect
    }
    fn build(&mut self, primitives: Vec<Primitive>) {
        self.primitives = primitives
    }
}
