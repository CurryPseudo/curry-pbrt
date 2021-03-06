use crate::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::VecDeque;
use std::{path::Path, sync::Mutex};

pub fn render(
    scene: Scene,
    sampler: Box<dyn Sampler>,
    integrator: Box<dyn Integrator>,
    film: Film,
    camera: Box<dyn Camera>,
) -> Film {
    let film_tiles = film.gen_tiles();
    let len = film_tiles.len() as u64;
    let progress_bar = ProgressBar::new(len);
    progress_bar.set_style(ProgressStyle::default_bar().template("{wide_bar} {percent}% ({eta_precise})"));
    progress_bar.enable_steady_tick(1000);
    let film = Mutex::new(film);
    film_tiles.into_par_iter().for_each(|mut tile| {
        //film_tiles.into_iter().for_each(|mut tile| {
        let mut sampler = sampler.box_clone();
        for film_point in tile.bound().index_inside() {
            trace!("Rendering point {}", film_point);
            sampler.set_pixel(&film_point);
            let mut film_point_f = Point2f::new(film_point.x as Float, film_point.y as Float);
            let sample_per_pixel = sampler.get_sample_per_pixel();
            let mut samples = Vec::new();
            for _i in 0..sample_per_pixel {
                let offset = sampler.get_2d() - Point2f::new(0.5, 0.5);
                film_point_f += offset;
                let ray = camera.generate_ray(film_point_f, sampler.as_mut());
                let li = integrator.li(&ray, &scene, sampler.as_mut());
                if li.has_nan() {
                    warn!("li has nan at pixel {} sample {}", film_point, _i);
                    warn!("");
                }
                else {
                    samples.push((offset, li));
                }
                sampler.next_sample();
            }
            tile.add_samples(&film_point, &samples);
        }
        film.lock().unwrap().merge_tile(tile);
        progress_bar.inc(1);
    });
    progress_bar.finish_and_clear();
    film.into_inner().unwrap()
}

fn parse_find_eat<'a, R: ParseFromBlockSegment<'a>>(
    segments: &'a mut VecDeque<BlockSegment>,
) -> Option<R::T> {
    for segment in segments {
        if let Some(r) = R::parse_from_segment(segment) {
            return Some(r);
        }
    }
    None
}

pub fn render_from_file(path: &Path) {
    let mut segments = read_scene(path).into_iter().collect::<VecDeque<_>>();
    let camera_transform = parse_find_eat::<Transform>(&mut segments);
    let camera_factory = parse_find_eat::<Box<dyn Camera>>(&mut segments).unwrap();
    let sampler_factory = parse_find_eat::<Box<dyn Sampler>>(&mut segments).unwrap();
    let (film, file_name, resolution) = parse_find_eat::<Film>(&mut segments).unwrap();
    let mut camera = camera_factory(resolution);
    if let Some(transform) = camera_transform {
        camera = camera_apply(camera, &transform);
    }
    let sampler = sampler_factory(resolution);
    let integrator = parse_find_eat::<Box<dyn Integrator>>(&mut segments).unwrap();
    let aggregate = Box::new(BVHAggregate::default());
    let scene_builder = parse_find_eat::<SceneBuilder>(&mut segments).unwrap();
    let mut scene = scene_builder.build_with_clipper(Some(camera.as_clipper()));
    scene.build_aggregate(aggregate);
    let film = render(scene, sampler, integrator, film, camera);
    film.write_image(&Path::new(file_name.as_str()));
    println!("{}", file_name);
}
