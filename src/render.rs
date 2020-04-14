use std::collections::VecDeque;
use crate::scene_file_parser::read_scene;
use std::path::Path;
use crate::{
    camera::{parse_camera, Camera}, def::Float, film::{parse_film, Film}, geometry::Point2f, integrator::{parse_integrator, Integrator},
    sampler::{SamplerWrapper, HaltonSampler, parse_sampler}, scene::{parse_scene, Scene},
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

pub fn render_from_file(path: &Path) {
    let mut segments = read_scene(path).into_iter().collect::<VecDeque<_>>();
    let camera_factory = parse_camera(&segments.pop_front().unwrap()).unwrap();
    let sampler = parse_sampler(&segments.pop_front().unwrap()).unwrap();
    let (mut film, file_name, resolution) = parse_film(&segments.pop_front().unwrap()).unwrap();
    let camera = camera_factory(resolution);
    let integrator = parse_integrator(&segments.pop_front().unwrap()).unwrap();
    let scene = parse_scene(&segments.pop_front().unwrap());
    render(scene, sampler, integrator, &mut film, camera);
    film.write_image(&Path::new(file_name.as_str()))
}
