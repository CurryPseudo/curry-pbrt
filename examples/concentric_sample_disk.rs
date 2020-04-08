use curry_pbrt::{geometry::concentric_sample_disk, sampler::{HaltonSampler, SamplerWrapper}};
fn main() {
    let mut halton = HaltonSampler::new(128);
    let mut sample = SamplerWrapper::from(&mut halton);
    for _ in 0..128 {
        let p = concentric_sample_disk(sample.get_2d());
        println!("{}", p.x);
        println!("{}", p.y);
        sample = sample.next_sample();
    }
}
