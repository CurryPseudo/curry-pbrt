use super::{Point, Point2u, Ray, Transform, Transformable, Vector, Vector3f};
use crate::*;
use alga::general::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use nalgebra::{
    allocator::Allocator,
    base::dimension::{U2, U3},
    DefaultAllocator, DimName, Scalar, Vector3,
};
use num_traits::{Bounded, FromPrimitive};
use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, Index},
};

pub trait BoundsTrait:
    Scalar + Copy + PartialOrd + Bounded + FromPrimitive + ClosedAdd + ClosedSub + ClosedMul + ClosedDiv
{
}
impl BoundsTrait for Integer {}
impl BoundsTrait for usize {}
impl BoundsTrait for Float {}

#[derive(Debug, Clone)]
pub struct Bounds<T: BoundsTrait, N: DimName>
where
    DefaultAllocator: Allocator<T, N>,
{
    pub min: Point<T, N>,
    pub max: Point<T, N>,
}

impl<T: BoundsTrait + Display, N: DimName> Display for Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} => {}", self.min, self.max)
    }
}

impl<T: BoundsTrait, N: DimName> Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    pub fn new(min: Point<T, N>, max: Point<T, N>) -> Self {
        Self { min, max }
    }
    pub fn corner(&self, index: usize) -> Point<T, N> {
        let dim = N::dim();
        let mut point_data = Vec::new();
        for i in 0..dim {
            let bounds_index = (index & (1 << i)) >> i;
            point_data.push(self[bounds_index][i].clone());
        }
        Point::from_slice(&point_data)
    }
    pub fn diagonal(&self) -> Vector<T, N> {
        &self.max - &self.min
    }
    pub fn maximum_extent(&self) -> usize {
        self.diagonal().imax()
    }
    pub fn lerp(&self, mut p: Point<T, N>) -> Point<T, N> {
        let dim = N::dim();
        for i in 0..dim {
            p[i] = lerp(p[i], self.min[i], self.max[i]);
        }
        p
    }
    pub fn offset(&self, p: Point<T, N>) -> Point<T, N> {
        let mut o = p - self.min.clone();
        let d = self.diagonal();
        for i in 0..N::dim() {
            if d[i] > T::from_i32(0).unwrap() {
                o[i] /= d[i];
            }
        }
        Point::from(o)
    }
    pub fn expand(mut self, delta: T) -> Self {
        self.min = self.min - Vector::from_element(delta.clone());
        self.max = self.max + Vector::from_element(delta);
        self
    }
    pub fn overlaps(&self, rhs: &Self) -> bool {
        let mut is_overlaps = true;
        let dim = N::dim();
        for i in 0..dim {
            if !is_overlaps {
                break;
            }
            is_overlaps = is_overlaps && self.min[i] <= rhs.max[i] && self.max[i] >= rhs.min[i];
        }
        is_overlaps
    }
    pub fn inside(&self, rhs: &Point<T, N>) -> bool {
        let mut is_overlaps = true;
        let dim = N::dim();
        for i in 0..dim {
            if !is_overlaps {
                break;
            }
            is_overlaps = is_overlaps && self.min[i] <= rhs[i] && self.max[i] >= rhs[i];
        }
        is_overlaps
    }
    pub fn inside_exclusive(&self, rhs: &Point<T, N>) -> bool {
        let mut is_overlaps = true;
        let dim = N::dim();
        for i in 0..dim {
            if !is_overlaps {
                break;
            }
            is_overlaps = is_overlaps && self.min[i] < rhs[i] && self.max[i] > rhs[i];
        }
        is_overlaps
    }
}

impl<T: BoundsTrait, N: DimName> From<Point<T, N>> for Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    fn from(p: Point<T, N>) -> Self {
        Self::new(p.clone(), p)
    }
}
impl<T: BoundsTrait, N: DimName> Default for Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    fn default() -> Self {
        let min = T::min_value();
        let max = T::max_value();
        Self {
            min: Point::from(Vector::from_element(min)),
            max: Point::from(Vector::from_element(max)),
        }
    }
}
impl<T: BoundsTrait, N: DimName> Index<usize> for Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    type Output = Point<T, N>;
    fn index(&self, index: usize) -> &Self::Output {
        let index = index % 2;
        if index == 0 {
            &self.min
        } else {
            &self.max
        }
    }
}

impl<T: BoundsTrait, N: DimName> BitOr<&Point<T, N>> for Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    type Output = Self;
    fn bitor(mut self, rhs: &Point<T, N>) -> Self::Output {
        let dim = N::dim();
        for i in 0..dim {
            self.min[i] = min(self.min[i], rhs[i]);
            self.max[i] = max(self.max[i], rhs[i]);
        }
        self
    }
}
impl<T: BoundsTrait, N: DimName> BitOr<&Self> for Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    type Output = Self;
    fn bitor(mut self, rhs: &Self) -> Self::Output {
        let dim = N::dim();
        for i in 0..dim {
            self.min[i] = min(self.min[i], rhs.min[i]);
            self.max[i] = max(self.max[i], rhs.max[i]);
        }
        self
    }
}
impl<T: BoundsTrait, N: DimName> BitAnd<&Self> for Bounds<T, N>
where
    DefaultAllocator: Allocator<T, N>,
{
    type Output = Self;
    fn bitand(mut self, rhs: &Self) -> Self::Output {
        let dim = N::dim();
        for i in 0..dim {
            self.min[i] = max(self.min[i], rhs.min[i]);
            self.max[i] = min(self.max[i], rhs.max[i]);
        }
        self
    }
}

pub type Bounds2<T> = Bounds<T, U2>;

impl<T: BoundsTrait> Bounds2<T> {
    pub fn area(&self) -> T {
        let d = self.diagonal();
        d.x * d.y
    }
}
pub type Bounds3<T> = Bounds<T, U3>;

impl<T: BoundsTrait> Bounds3<T> {
    pub fn volume(&self) -> T {
        let d = self.diagonal();
        d.x * d.y * d.z
    }
    pub fn surface_area(&self) -> T {
        let d = self.diagonal();
        return T::from_i32(2).unwrap() * (d.x * d.y + d.x * d.z + d.y * d.z);
    }
}

pub type Bounds2i = Bounds<Integer, U2>;

pub type Bounds2u = Bounds<usize, U2>;

impl Bounds2u {
    pub fn index_inside(&self) -> Vec<Point2u> {
        let mut r = Vec::new();
        for i in self.min.x..self.max.x {
            for j in self.min.y..self.max.y {
                r.push(Point2u::new(i, j))
            }
        }
        r
    }
    pub fn point_to_offset(&self, point: &Point2u) -> usize {
        let o = point - self.min;
        o.x + o.y * self.diagonal().x
    }
}
pub type Bounds2f = Bounds<Float, U2>;
pub type Bounds3i = Bounds<Integer, U3>;
pub type Bounds3f = Bounds<Float, U3>;

fn is_positive(f: Float) -> usize {
    if f >= 0. {
        1
    } else {
        0
    }
}

impl Transformable for Bounds3f {
    fn apply(self, transform: &Transform) -> Self {
        let mut r = Self::from(self.min.apply(transform));
        for i in 1..8 {
            r = r | &self.corner(i).apply(transform)
        }
        r
    }
}

impl Bounds3f {
    pub fn intersect_predicate(&self, ray: &Ray) -> bool {
        self.intersect_predicate_cached(&RayIntersectCache::from(ray.clone()))
    }
    fn intersect_component(&self, ray: &RayIntersectCache, c: usize) -> Option<(Float, Float)> {
        if ray.ray.d[c] == 0. {
            None
        } else {
            Some((
                (self[ray.is_negative_d[c]][c] - ray.ray.o[c]) * ray.inverse_d[c],
                (self[ray.is_positive_d[c]][c] - ray.ray.o[c]) * ray.inverse_d[c],
            ))
        }
    }
    pub fn intersect_predicate_cached(&self, ray: &RayIntersectCache) -> bool {
        let mut pair = None;
        for i in 0..3 {
            if let Some((t_min, t_max)) = &mut pair {
                if let Some((t_next_min, t_next_max)) = self.intersect_component(ray, i) {
                    if *t_min > t_next_max || t_next_min > *t_max {
                        return false;
                    }
                    *t_min = min(*t_min, t_next_min);
                    *t_max = max(*t_max, t_next_max);
                }
            } else {
                pair = self.intersect_component(ray, i);
            }
        }
        if let Some((t_min, t_max)) = pair {
            t_min < ray.ray.t_max && t_max > 0.
        }
        else {
            false
        }
    }
}

pub struct RayIntersectCache {
    ray: Ray,
    inverse_d: Vector3f,
    is_positive_d: Vector3<usize>,
    is_negative_d: Vector3<usize>,
}

impl Display for RayIntersectCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} inverse_d {} is_positive {} is_negative {}",
            self.ray, self.inverse_d, self.is_positive_d, self.is_negative_d
        )
    }
}
impl RayIntersectCache {
    pub fn origin_ray(&self) -> &Ray {
        &self.ray
    }
}

impl From<Ray> for RayIntersectCache {
    fn from(ray: Ray) -> Self {
        let d = ray.d;
        let inverse_d = Vector3f::new(1. / d.x, 1. / d.y, 1. / d.z);
        let is_positive_d = Vector3::new(is_positive(d.x), is_positive(d.y), is_positive(d.z));
        let is_negative_d = Vector3::from_element(1) - is_positive_d;
        Self {
            ray,
            inverse_d,
            is_positive_d,
            is_negative_d,
        }
    }
}
