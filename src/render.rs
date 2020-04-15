use crate::scene_file_parser::read_scene;
use crate::{
    camera::{parse_camera, Camera},
    def::Float,
    film::{parse_film, Film, Renderable},
    geometry::Point2f,
    integrator::{parse_integrator, Integrator},
    sampler::{parse_sampler, SamplerWrapper},
    scene::{parse_scene, Scene},
};
use rayon::prelude::*;
use std::collections::VecDeque;
use std::{sync::Mutex, path::Path};
use indicatif::{ProgressStyle, ProgressBar};

pub fn render(
    scene: Scene,
    sampler: SamplerWrapper,
    integrator: Box<dyn Integrator>,
    film: Film,
    camera: Box<dyn Camera>,
) -> Film {
    let film_tiles = film.gen_tiles();
    let progress_bar = ProgressBar::new(film.bound().area() as u64);
    progress_bar.set_style(ProgressStyle::default_bar().template("{bar} ({eta})"));
    let film = Mutex::new(film);
    film_tiles.into_par_iter().for_each(|mut tile| {
        let mut sampler = sampler.clone().set_pixel(tile.get_pixel_begin_index());
        for film_point in tile.bound().index_inside() {
            let mut film_point_f = Point2f::new(film_point.x as Float, film_point.y as Float);
            let sample_per_pixel = sampler.get_sample_per_pixel();
            let mut samples = Vec::new();
            for _ in 0..sample_per_pixel {
                let offset = sampler.get_2d() - Point2f::new(0.5, 0.5);
                film_point_f += offset;
                let ray = camera.generate_ray(film_point_f);
                let li = integrator.li(&ray, &scene, &mut sampler);
                samples.push((offset, li));
                sampler = sampler.next_sample();
            }
            tile.add_samples(&film_point, &samples);
            sampler = sampler.next_pixel();
            progress_bar.inc(1);
        }
        film.lock().unwrap().merge_tile(tile);
    });
    progress_bar.finish_and_clear();
    film.into_inner().unwrap()
}

pub fn render_from_file(path: &Path) {
    let mut segments = read_scene(path).into_iter().collect::<VecDeque<_>>();
    let camera_factory = parse_camera(&segments.pop_front().unwrap()).unwrap();
    let sampler = parse_sampler(&segments.pop_front().unwrap()).unwrap();
    let (film, file_name, resolution) = parse_film(&segments.pop_front().unwrap()).unwrap();
    let camera = camera_factory(resolution);
    let integrator = parse_integrator(&segments.pop_front().unwrap()).unwrap();
    let scene = parse_scene(&segments.pop_front().unwrap());
    let film = render(scene, sampler, integrator, film, camera);
    film.write_image(&Path::new(file_name.as_str()))
}
