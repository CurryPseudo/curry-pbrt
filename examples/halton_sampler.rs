use curry_pbrt::{geometry::Vector2u, sampler::{HaltonSampler, Sampler}};
use std::sync::Mutex;

fn main() {
    let mut sampler = Box::new(HaltonSampler::new(128, Vector2u::new(1024, 768)));
    let count = 128;
    for _ in 0..count {
        let point = sampler.get_2d();
        println!("{}", point.x);
        println!("{}", point.y);
        sampler.next_sample();
    }
}
