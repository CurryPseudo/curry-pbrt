use crate::{
    camera::Camera, def::Float, film::Film, geometry::Point2f, integrator::Integrator,
    sampler::{SamplerWrapper, HaltonSampler}, scene::Scene,
};
use std::sync::Mutex;

pub fn render(
    scene: Scene,
    mut sampler: SamplerWrapper,
    integrator: Box<dyn Integrator>,
    film: &mut Film,
    camera: Box<dyn Camera>,
) {
    for film_point in film.bound().index_inside() {
        let film_point_f = Point2f::new(film_point.x as Float, film_point.y as Float);
        let ray = camera.generate_ray(film_point_f);
        let li = integrator.li(&ray, &scene, &mut sampler);
        film.add_sample(&film_point, li);
        sampler = sampler.next_pixel();
    }
}
