use crate::math::clamp;
use crate::{
    def::Float,
    geometry::{Bounds2u, Point2u, Vector2u},
    scene_file_parser::BlockSegment,
    spectrum::Spectrum,
};
use png::HasParameters;
use png::{BitDepth, ColorType, Encoder};
use std::{collections::VecDeque, fs::File, io::BufWriter, path::Path};

pub struct Film {
    pub(crate) resolution: Vector2u,
    pixels: Vec<Spectrum>,
}

impl Film {
    pub fn new(resolution: Vector2u) -> Self {
        let pixels = vec![Spectrum::new(0.); resolution.x * resolution.y];
        Self { resolution, pixels }
    }
    pub fn bound(&self) -> Bounds2u {
        Bounds2u::new(Point2u::new(0, 0), Point2u::from(self.resolution))
    }
    fn point_to_index(&self, point: &Point2u) -> usize {
        point.x + point.y * self.resolution.x
    }
    pub fn add_sample(&mut self, point: &Point2u, spectrum: Spectrum) {
        let index = self.point_to_index(point);
        self.pixels[index] += spectrum;
    }
    pub fn write_image(self, file_path: &Path) {
        let file = File::create(file_path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = Encoder::new(w, self.resolution.x as u32, self.resolution.y as u32);
        encoder.set(ColorType::RGB).set(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut data = Vec::new();
        for pixel in self.pixels {
            let [r, g, b]: [Float; 3] = pixel.into();
            data.push(clamp(r * 255., 0., 255.) as u8);
            data.push(clamp(g * 255., 0., 255.) as u8);
            data.push(clamp(b * 255., 0., 255.) as u8);
        }
        writer.write_image_data(&data).unwrap()
    }
}
pub fn parse_film(segment: &BlockSegment) -> Option<(Film, String, Vector2u)> {
    let property_set = segment.get_object_by_type("Film").unwrap();
    if property_set.get_name().unwrap() == "image" {
        let x_resolution = property_set
            .get_integer("xresolution")
            .unwrap_or(640);
        let y_resolution = property_set
            .get_integer("yresolution")
            .unwrap_or(480);
        let resolution = Vector2u::new(x_resolution as usize, y_resolution as usize);
        let file_name = String::from(
            property_set
                .get_string("filename")
                .unwrap_or("curry-pbrt.png"),
        );
        Some((Film::new(resolution), file_name, resolution))
    }
    else {
        panic!()
    }
    
}
