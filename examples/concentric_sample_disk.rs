use curry_pbrt::{geometry::concentric_sample_disk, sampler::{HaltonSampler, SamplerWrapper}};
use std::sync::Mutex;
fn main() {
    let mut sampler = SamplerWrapper::new(Box::new(HaltonSampler::new()), 128);
    for _ in 0..128 {
        let p = concentric_sample_disk(sampler.get_2d());
        println!("{}", p.x);
        println!("{}", p.y);
        sampler = sampler.next_sample();
    }
}
