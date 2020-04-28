use crate::texture::image::exr::ExrImageFileReader;
use crate::*;
use std::fs::File;
use std::path::Path;
mod exr;
mod png;
use crate::texture::image::png::PngImageFileReader;

pub trait ImageTextureContent {
    fn default() -> Self;
    fn from_rgb_spectrum(s: RGBSpectrum) -> Self;
    fn into_float(self) -> Float;
    fn apply_inverse_gamma_correct(self) -> Self;
}

impl ImageTextureContent for Float {
    fn default() -> Self {
        0.
    }
    fn from_rgb_spectrum(s: RGBSpectrum) -> Self {
        s.y()
    }
    fn into_float(self) -> Float {
        self
    }
    fn apply_inverse_gamma_correct(self) -> Self {
        inverse_gamma_correct(self)
    }
}

impl ImageTextureContent for Spectrum {
    fn default() -> Self {
        Spectrum::new(0.)
    }
    fn from_rgb_spectrum(s: RGBSpectrum) -> Self {
        s
    }
    fn into_float(self) -> Float {
        self.y()
    }
    fn apply_inverse_gamma_correct(mut self) -> Self {
        self.map(|f| *f = inverse_gamma_correct(*f));
        self
    }
}

#[derive(Debug, Clone)]
pub struct ImageTexture<T> {
    pixels: FixedVec2D<T>,
}

impl<T: Clone + ImageTextureContent + Default> ImageTexture<T> {
    pub fn apply_inverse_gamma_correct(&mut self) {
        let pixels = std::mem::take(&mut self.pixels);
        self.pixels = pixels.map(|t| t.apply_inverse_gamma_correct());
    }
}

pub trait ImageFileReader {
    fn read_file(&self, file_path: &Path) -> (Vector2u, Vec<Spectrum>);
}
impl<T: Clone + ImageTextureContent> ImageTexture<T> {
    pub fn from_file(file_path: &Path) -> Self {
        let image_file_reader: Box<dyn ImageFileReader> =
            match file_path.extension().unwrap().to_str().unwrap() {
                "png" => Box::new(PngImageFileReader {}),
                "exr" => Box::new(ExrImageFileReader {}),
                _ => panic!(),
            };
        let (resolution, buf) = image_file_reader.read_file(file_path);
        let mut vec = Vec::new();
        for spectrum in buf {
            vec.push(T::from_rgb_spectrum(spectrum));
        }
        Self {
            pixels: FixedVec2D::from_vec(vec, resolution.x),
        }
    }
}

impl<T: ImageTextureContent + Clone + std::marker::Sync + std::marker::Send + std::fmt::Debug>
    Texture<T> for ImageTexture<T>
{
    fn evaluate(&self, uv: &Point2f) -> T {
        let size = self.pixels.size();
        let mut uv = uv.coords;
        uv.y = 1. - uv.y;
        let i = uv
            .component_mul(&size.map(|u| u as Float))
            .map(|f| f as usize)
            .zip_map(&size, |x, size| min(x, size - 1));
        self.pixels[i.into()].clone()
    }
    fn pixels(&self) -> FixedVec2D<T> {
        self.pixels.clone()
    }
}
