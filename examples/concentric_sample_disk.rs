use curry_pbrt::{geometry::concentric_sample_disk, sampler::{HaltonSampler, SamplerWrapper}};
use std::sync::Mutex;
fn main() {
    let halton = Mutex::new(HaltonSampler::new());
    let mut sample = SamplerWrapper::new(&halton, 128);
    for _ in 0..128 {
        let p = concentric_sample_disk(sample.get_2d());
        println!("{}", p.x);
        println!("{}", p.y);
        sample = sample.next_sample();
    }
}
