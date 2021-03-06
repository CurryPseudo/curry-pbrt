use curry_pbrt::*;
fn main() {
    let mut sampler = Box::new(HaltonSampler::new(128, Vector2u::new(1024, 768)));
    for _ in 0..128 {
        let p = concentric_sample_disk(sampler.get_2d());
        println!("{}", p.x);
        println!("{}", p.y);
        sampler.next_sample();
    }
}
