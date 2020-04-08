use curry_pbrt::sampler::{SamplerWrapper, HaltonSampler};

fn main() {
    let mut halton_sampler = HaltonSampler::new(128);
    let mut sampler = SamplerWrapper::new(&mut halton_sampler, 166, 0, 0);
    let count = 128;
    for _ in 0..count {
        let point = sampler.get_2d();
        println!("{}", point.x);
        println!("{}", point.y);
        sampler = sampler.next_sample();
    }
}
