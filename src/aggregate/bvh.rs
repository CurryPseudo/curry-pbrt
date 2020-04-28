use crate::*;
use indicatif::{ProgressBar, ProgressStyle};
use ordered_float::OrderedFloat;
use std::ops::Range;
#[derive(Default)]
pub struct BVHAggregate {
    primitives: Vec<Primitive>,
    nodes: Vec<BVHNode>,
    root: usize,
}

#[derive(Debug)]
pub enum BVHNode {
    Leaf(Range<usize>),
    Parent {
        axis: usize,
        left: usize,
        right: usize,
        bound: Bounds3f,
    },
}

impl BVHAggregate {
    fn create_sah_node<F: Fn(usize)>(
        &mut self,
        range: Range<usize>,
        create_leaf_callback: &F,
    ) -> usize {
        let len = range.end - range.start;
        assert!(len > 0);
        if len <= 1 {
            let r = self.nodes.len();
            self.nodes.push(BVHNode::Leaf(range));
            create_leaf_callback(len);
            return r;
        }
        let full_bound = {
            let mut full_bound: Option<Bounds3f> = None;
            for primitive in &self.primitives[range.clone()] {
                if let Some(full_bound) = full_bound.as_mut() {
                    *full_bound |= primitive.bound();
                } else {
                    full_bound = Some(primitive.bound().clone());
                }
            }
            full_bound.unwrap()
        };
        let start = range.start;
        let mut min_cost: Option<(Float, Vec<usize>, usize, usize)> = None;
        for axis in 0..3 {
            let mut sort_by_axis_index: Vec<usize> = (0..len).collect();
            sort_by_axis_index.sort_by_cached_key(|i| {
                OrderedFloat::from(self.primitives[*i + start].bound().center()[axis])
            });
            let mut bound_forward: Vec<Bounds3f> = Vec::new();
            let mut bound_backward: Vec<Bounds3f> = Vec::new();
            for i in 0..(len - 1) {
                let this_forward_bound = self.primitives[sort_by_axis_index[i] + start].bound();
                let this_backward_bound =
                    self.primitives[sort_by_axis_index[len - 1 - i] + start].bound();
                if let Some(last_forward) = bound_forward.last() {
                    let last_forward = last_forward.clone();
                    bound_forward.push(last_forward | &this_forward_bound);
                    bound_backward
                        .push(bound_backward.last().unwrap().clone() | &this_backward_bound);
                } else {
                    bound_forward.push(this_forward_bound.clone());
                    bound_backward.push(this_backward_bound.clone());
                }
            }
            bound_backward.reverse();
            for i in 0..(len - 1) {
                let cost = 0.125
                    + ((i + 1) as Float * bound_forward[i].surface_area()
                        + (len - 1 - i) as Float * bound_backward[i].surface_area())
                        / full_bound.surface_area();
                if let Some((min_cost, min_sorted_indices, min_axis, min_i)) = min_cost.as_mut() {
                    if cost < *min_cost {
                        *min_cost = cost;
                        if axis != *min_axis {
                            *min_axis = axis;
                            *min_sorted_indices = sort_by_axis_index.clone();
                        }
                        *min_i = i
                    }
                } else {
                    min_cost = Some((cost, sort_by_axis_index.clone(), axis, i));
                }
            }
        }
        let (min_cost, min_sorted_indices, min_axis, min_i) = min_cost.unwrap();
        if min_cost > len as Float {
            // construct leaf
            let r = self.nodes.len();
            self.nodes.push(BVHNode::Leaf(range));
            create_leaf_callback(len);
            debug!("leaf has {} node", len);
            return r;
        }
        let mut swapped = vec![false; len];
        for i in 0..len {
            if !swapped[i] {
                let mut j = i;
                let mut k = min_sorted_indices[j];
                while k != i {
                    self.primitives.swap(j + start, k + start);
                    swapped[k] = true;
                    j = k;
                    k = min_sorted_indices[k];
                }
            }
        }
        let left = self.create_sah_node(start..start + min_i + 1, create_leaf_callback);
        let right = self.create_sah_node(start + min_i + 1..start + len, create_leaf_callback);
        let node = BVHNode::Parent {
            axis: min_axis,
            left,
            right,
            bound: full_bound,
        };
        let r = self.nodes.len();
        self.nodes.push(node);
        r
    }
    fn intersect_predicate_through_bound(&self, node: usize, ray: &RayIntersectCache) -> bool {
        match &self.nodes[node] {
            BVHNode::Leaf(range) => {
                for primitive in &self.primitives[range.clone()] {
                    if primitive.intersect_predicate(ray.origin_ray()) {
                        return true;
                    }
                }
                false
            }
            BVHNode::Parent {
                axis: _,
                left,
                right,
                bound,
            } => {
                if bound.intersect_predicate_cached(ray)
                    && (self.intersect_predicate_through_bound(*left, ray)
                        || self.intersect_predicate_through_bound(*right, ray))
                {
                    return true;
                }
                false
            }
        }
    }
    fn intersect_through_bound(
        &self,
        node: usize,
        ray: &mut RayIntersectCache,
    ) -> Option<PrimitiveIntersect> {
        let mut result = None;
        match &self.nodes[node] {
            BVHNode::Leaf(range) => {
                for primitive in &self.primitives[range.clone()] {
                    if let Some(intersect) = primitive.intersect(ray.origin_ray()) {
                        ray.update_t_max(intersect.get_shape_intersect().get_t());
                        result = Some(intersect);
                    }
                }
            }
            BVHNode::Parent {
                axis,
                left,
                right,
                bound,
            } => {
                if bound.intersect_predicate_cached(ray) {
                    // first right, then left
                    let (left, right) = if ray.is_positive_d[*axis] == 1 {
                        (left, right)
                    } else {
                        (right, left)
                    };

                    if let Some(intersect) = self.intersect_through_bound(*right, ray) {
                        result = Some(intersect);
                    }
                    if let Some(intersect) = self.intersect_through_bound(*left, ray) {
                        result = Some(intersect);
                    }
                }
            }
        }
        result
    }
}

impl Aggregate for BVHAggregate {
    fn build(&mut self, primitives: Vec<Primitive>) {
        if primitives.is_empty() {
            return;
        }
        let progress_bar = ProgressBar::new(primitives.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("Building BVH {wide_bar} {percent}% ({eta_pricise})"),
        );
        self.primitives = primitives;
        self.root = self.create_sah_node(0..self.primitives.len(), &|leaf_count| {
            progress_bar.inc(leaf_count as u64)
        });
    }
    fn intersect_predicate(&self, ray: &Ray) -> bool {
        if self.nodes.is_empty() {
            return false;
        }
        let ray = RayIntersectCache::from(*ray);
        self.intersect_predicate_through_bound(self.root, &ray)
    }
    fn intersect(&self, ray: &Ray) -> Option<PrimitiveIntersect> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut ray = RayIntersectCache::from(*ray);
        self.intersect_through_bound(self.root, &mut ray)
    }
}
