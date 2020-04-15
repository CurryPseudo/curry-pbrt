use curry_pbrt::sampler::{SamplerWrapper, HaltonSampler};
use std::sync::Mutex;

fn main() {
    let mut sampler = SamplerWrapper::new(Box::new(HaltonSampler::new()), 128);
    let count = 128;
    for _ in 0..count {
        let point = sampler.get_2d();
        println!("{}", point.x);
        println!("{}", point.y);
        sampler = sampler.next_sample();
    }
}
