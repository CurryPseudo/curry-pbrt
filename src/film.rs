use crate::*;
use png::HasParameters;
use png::{BitDepth, ColorType, Encoder};
use std::{fmt::Debug, fs::File, io::BufWriter, path::Path};

pub trait Renderable {
    fn bound(&self) -> &Bounds2u;
    fn get_pixels(&mut self) -> &mut FixedVec2D<Spectrum>;
    fn add_sample(&mut self, point: &Point2u, spectrum: Spectrum) {
        let i = Point2u::new(0, 0)+ (point - &self.bound().min);
        self.get_pixels()[i] += spectrum;
    }
    fn add_samples(&mut self, point: &Point2u, sampels: &[(Vector2f, Spectrum)]) {
        let mut sum = Spectrum::new(0.);
        for sample in sampels {
            sum += sample.1;
        }
        sum /= sampels.len() as Float;
        self.add_sample(point, sum);
    }
}

pub struct Film {
    pixels: FixedVec2D<Spectrum>,
    bound: Bounds2u,
}

impl Film {
    pub fn new(resolution: Vector2u) -> Self {
        let pixels = FixedVec2D::new(Spectrum::new(0.), resolution);
        let bound = Bounds2u::new(&Point2u::new(0, 0), &Point2u::from(resolution));
        Self {
            pixels,
            bound,
        }
    }
    pub fn write_image(self, file_path: &Path) {
        let file = File::create(file_path).unwrap();
        let ref mut w = BufWriter::new(file);
        let resolution = self.pixels.size();
        let mut encoder = Encoder::new(w, resolution.x as u32, resolution.y as u32);
        encoder.set(ColorType::RGB).set(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut data = Vec::new();
        let mut floats = Vec::new();
        for pixel in self.pixels {
            let [r, g, b]: [Float; 3] = pixel.into();
            floats.push(r);
            floats.push(g);
            floats.push(b);
        }
        let mut f_min = None;
        let mut f_max = None;
        for float in &floats {
            f_min = Some(f_min.map_or(*float, |f_min| min(f_min, *float)));
            f_max = Some(f_max.map_or(*float, |f_max| max(f_max, *float)));
        }
        let _f_min = f_min.unwrap();
        let _f_max = f_max.unwrap();
        for float in floats {
            data.push(clamp(
                gamma_correct(float) * 255. + 0.5,
                //gamma_correct(rlerp(float, 0., _f_max)) * 255. + 0.5,
                0.,
                255.,
            ) as u8);
        }
        writer.write_image_data(&data).unwrap()
    }
    pub fn gen_tiles(&self) -> Vec<FilmTile> {
        let tile_size = 16;
        let resolution = self.pixels.size();
        let tile_indices = Bounds2u::new(
            &Point2u::new(0, 0),
            &Point2u::new(
                (resolution.x - 1) / tile_size + 1,
                (resolution.y - 1) / tile_size + 1,
            ),
        );
        let self_bound = self.bound();
        let mut r = Vec::new();
        for tile_index in tile_indices.index_inside() {
            let next = Point2u::new(tile_index.x + 1, tile_index.y + 1);
            let bound = Bounds2u::new(&(tile_index * tile_size), &(next * tile_size)) & &self_bound;
            r.push(FilmTile::new(
                bound
            ));
        }
        r
    }
    pub fn merge_tile(&mut self, tile: FilmTile) {
        for (p, s) in tile.into_merge() {
            self.add_sample(&p, s);
        }
    }
}
impl Renderable for Film {
    fn bound(&self) -> &Bounds2u {
        &self.bound
    }
    fn get_pixels(&mut self) -> &mut FixedVec2D<Spectrum> {
        &mut self.pixels
    }
}

impl ParseFromBlockSegment for Film {
    type T = (Film, String, Vector2u);
    fn parse_from_segment(segment: &BlockSegment) -> Option<Self::T> {
        let property_set = segment.get_object_by_type("Film").unwrap();
        if property_set.get_name().unwrap() == "image" {
            let x_resolution = property_set.get_value("xresolution").unwrap_or(640);
            let y_resolution = property_set.get_value("yresolution").unwrap_or(480);
            let resolution = Vector2u::new(x_resolution, y_resolution);
            let file_name = property_set
                .get_string("filename")
                .unwrap_or(String::from("curry-pbrt.png"));
            Some((Film::new(resolution), file_name, resolution))
        } else {
            panic!()
        }
    }
}

pub struct FilmTile {
    bound: Bounds2u,
    pixels: FixedVec2D<Spectrum>,
}

impl FilmTile {
    pub fn new(bound: Bounds2u) -> Self {
        let d = bound.diagonal();
        Self {
            bound,
            pixels: FixedVec2D::new(Spectrum::new(0.), d),
        }
    }
    pub fn into_merge(self) -> Vec<(Point2u, Spectrum)> {
        let mut index = 0;
        let d = self.bound.diagonal();
        let min = self.bound.min;
        self.pixels
            .into_iter()
            .map(|s| {
                let r = (Point2u::new(index % d.x, index / d.x) + min.coords, s);
                index += 1;
                r
            })
            .collect()
    }
}
impl Renderable for FilmTile {
    fn bound(&self) -> &Bounds2u {
        &self.bound
    }
    fn get_pixels(&mut self) -> &mut FixedVec2D<Spectrum> {
        &mut self.pixels
    }
}

impl Debug for FilmTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bound)
    }
}
