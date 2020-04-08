use crate::{def::Float, geometry::Point2f};
use std::sync::Mutex;

mod halton_sampler;
pub use halton_sampler::*;

pub trait Sampler {
    fn get_sample(&mut self, pixel_index: usize, sample_index: usize, dim: usize) -> Float;
}

pub struct SamplerWrapper<'a> {
    sampler: &'a mut dyn Sampler,
    pixel_index: usize,
    sample_index: usize,
    dim: usize,
}

impl<'a, T: Sampler> From<&'a mut T> for SamplerWrapper<'a> {
    fn from(sampler: &'a mut T) -> Self {
        Self {
            sampler,
            pixel_index: 0,
            sample_index: 0,
            dim: 0,
        }
    }
}

impl<'a> SamplerWrapper<'a> {
    pub fn new<T: Sampler>(
        sampler: &'a mut T,
        pixel_index: usize,
        sample_index: usize,
        dim: usize,
    ) -> Self {
        Self {
            sampler,
            pixel_index,
            sample_index,
            dim,
        }
    }

    pub fn next_pixel(self) -> Self {
        Self {
            sampler: self.sampler,
            pixel_index: self.pixel_index + 1,
            sample_index: 0,
            dim: 0,
        }
    }
    pub fn next_sample(self) -> Self {
        Self {
            sampler: self.sampler,
            pixel_index: self.pixel_index,
            sample_index: self.sample_index + 1,
            dim: 0,
        }
    }
    pub fn get_1d(&mut self) -> Float {
        let r = self
            .sampler
            .get_sample(self.pixel_index, self.sample_index, self.dim);
        self.dim += 1;
        if r == 1. {
            0.9999999999999
        } else {
            r
        }
    }
    pub fn get_2d(&mut self) -> Point2f {
        Point2f::new(self.get_1d(), self.get_1d())
    }
    pub fn get_1ds(&mut self, count: usize) -> Vec<Float> {
        (0..count).into_iter().map(|_| self.get_1d()).collect()
    }
    pub fn get_2ds(&mut self, count: usize) -> Vec<Point2f> {
        (0..count).into_iter().map(|_| self.get_2d()).collect()
    }
}
