use crate::*;
mod halton;
pub use halton::*;

pub trait Sampler: Sync + Send {
    fn get_sample(&mut self) -> Float;
    fn box_clone(&self) -> Box<dyn Sampler>;
    fn set_pixel(&mut self, pixel: &Point2u);
    fn next_sample(&mut self);
    fn get_1d(&mut self) -> Float {
        let r = self.get_sample();
        if r >= 1. {
            0.9999999999999
        } else {
            r
        }
    }
    fn get_distribution_1d_remap(&mut self, len: usize, f: &dyn Fn(usize) -> Float) -> (usize, Float, Float) {
        sample_distribution_1d_remap(self.get_1d(), len, f)
    }
    fn get_usize_remap(&mut self, max: usize) -> (usize, Float) {
        sample_usize_remap(self.get_1d(), max)
    }
    fn get_usize(&mut self, max: usize) -> usize {
        let f = self.get_sample() * (max as Float);
        let f_trunc = f.trunc();
        let r = f_trunc as usize;
        if r == max {
            max - 1
        } else {
            r
        }
    }
    fn get_2d(&mut self) -> Point2f {
        Point2f::new(self.get_1d(), self.get_1d())
    }
    fn get_1ds(&mut self, count: usize) -> Vec<Float> {
        (0..count).map(|_| self.get_1d()).collect()
    }
    fn get_2ds(&mut self, count: usize) -> Vec<Point2f> {
        (0..count).map(|_| self.get_2d()).collect()
    }
    fn get_sample_per_pixel(&self) -> usize;
}

impl ParseFromBlockSegment<'_> for Box<dyn Sampler> {
    type T = Box<dyn FnOnce(Vector2u) -> Box<dyn Sampler>>;
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T> {
        let property_set = segment.get_object_by_type("Sampler")?;
        if property_set.get_name().unwrap() == "halton" {
            let pixel_samples = property_set.get_value("pixelsamples").unwrap();
            Some(Box::new(move |resolution| {
                Box::new(HaltonSampler::new(pixel_samples, resolution))
            }))
        } else {
            panic!()
        }
    }
}
