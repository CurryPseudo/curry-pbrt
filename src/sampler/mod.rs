use crate::{def::Float, geometry::Point2f};
use std::sync::Mutex;

mod halton;
pub use halton::*;

#[derive(Clone)]
pub struct SamplerWrapper<'a> {
    sampler: &'a Mutex<HaltonSampler>,
    pixel_index: usize,
    sample_index: usize,
    sampler_per_pixel: usize,
    dim: usize,
}

impl<'a> SamplerWrapper<'a> {
    pub fn new(
        sampler: &'a Mutex<HaltonSampler>,
        sampler_per_pixel: usize
    ) -> Self {
        Self {
            sampler,
            pixel_index: 0,
            sample_index: 0,
            sampler_per_pixel,
            dim: 0,
        }
    }

    pub fn next_pixel(self) -> Self {
        let pixel_index = self.pixel_index + 1;
        self.set_pixel(pixel_index)
    }

    pub fn set_pixel(mut self, pixel_index: usize) -> Self {
        self.pixel_index = pixel_index;
        self.sample_index = 0;
        self.dim = 0;
        self
    }
    pub fn set_sample(mut self, sample_index: usize) -> Self {
        self.sample_index = sample_index;
        self.dim = 0;
        self
    }
    pub fn next_sample(self) -> Self {
        let sample_index = self.sample_index + 1;
        self.set_sample(sample_index)
    }
    pub fn get_1d(&mut self) -> Float {
        let r = self
            .sampler.lock().unwrap()
            .get_sample(self.pixel_index * self.sampler_per_pixel +  self.sample_index, self.dim);
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
