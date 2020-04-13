use crate::{geometry::{Vector2u, Point2u, Bounds2u}, spectrum::Spectrum, def::Float};
use std::{path::Path, fs::File, io::BufWriter};
use png::{ColorType, Encoder, BitDepth};
use png::HasParameters;

pub struct Film {
    pub(crate) resolution: Vector2u,
    pixels: Vec<Spectrum>,
}

impl Film {
    pub fn new(resolution: Vector2u) -> Self {
        let pixels =
            vec![Spectrum::new(0.); resolution.x * resolution.y];
        Self { resolution, pixels }
    }
    pub fn bound(&self) -> Bounds2u {
        Bounds2u::new(Point2u::new(0,0), Point2u::from(self.resolution))
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
            data.push((r * 255.) as u8);
            data.push((g * 255.) as u8);
            data.push((b * 255.) as u8);
        }
        writer.write_image_data(&data).unwrap()
    }
}
