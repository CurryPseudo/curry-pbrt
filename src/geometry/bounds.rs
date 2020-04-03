use super::{Point, Point2i, Transformable, Vector, Transform};
use crate::def::Float;
use crate::def::Integer;
use alga::general::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};
use nalgebra::{
    allocator::Allocator,
    base::dimension::{U2, U3},
    DefaultAllocator, DimName, Scalar,
};
use num_traits::{Bounded, FromPrimitive};
use std::ops::{BitAnd, BitOr, Index};

pub trait BoundsTrait:
    Scalar + Copy + PartialOrd + Bounded + FromPrimitive + ClosedAdd + ClosedSub + ClosedMul + ClosedDiv
{
}
impl BoundsTrait for Integer {}
impl BoundsTrait for Float {}

#[derive(Debug, Clone)]
pub struct Bounds<T: BoundsTrait, N: DimName>
where
    DefaultAllocator: Allocator<T, N>,
{
    min: Point<T, N>,
    max: Point<T, N>,
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

impl Bounds2i {
    pub fn index_inside(&self) -> Vec<Point2i> {
        let mut r = Vec::new();
        for i in self.min.x..self.max.x {
            for j in self.min.y..self.max.y {
                r.push(Point2i::new(i, j))
            }
        }
        r
    }
}

pub type Bounds2f = Bounds<Float, U2>;
pub type Bounds3i = Bounds<Integer, U3>;
pub type Bounds3f = Bounds<Float, U3>;

fn lerp<T: BoundsTrait>(t: T, min: T, max: T) -> T {
    min * t + max * (T::from_i32(1).unwrap() - t)
}

fn min<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if rhs < lhs {
        rhs
    } else {
        lhs
    }
}
fn max<T: PartialOrd>(lhs: T, rhs: T) -> T {
    if rhs > lhs {
        rhs
    } else {
        lhs
    }
}

impl Transformable for Bounds3f {
    fn apply(self, transform: &Transform) -> Self {
        let mut r = Self::from(self.min.apply(transform));
        for i in 1..8 {
            r = r | &self.corner(i)
        }
        r
    }
}
