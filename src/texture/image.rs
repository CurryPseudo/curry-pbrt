use crate::*;
use std::fs::File;
use std::path::Path;

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
impl<T: Clone + ImageTextureContent> ImageTexture<T> {
    pub fn from_file(file: &Path) -> Self {
        if let Ok(file) = File::open(file) {
            let decoder = png::Decoder::new(file);
            let (info, mut reader) = decoder.read_info().unwrap();
            let mut buf = vec![0; info.buffer_size()];
            reader.next_frame(&mut buf).unwrap();
            let resolution = Vector2u::new(info.width as usize, info.height as usize);
            let mut vec = Vec::new();
            for i in 0..buf.len() / 3 {
                let mut rgb = Vec::new();
                for j in 0..3 {
                    rgb.push(buf[i * 3 + j] as Float / 255.);
                }
                vec.push(T::from_rgb_spectrum(RGBSpectrum::from([
                    rgb[0], rgb[1], rgb[2],
                ])));
            }
            Self {
                pixels: FixedVec2D::from_vec(vec, resolution.x),
            }
        } else {
            Self {
                pixels: FixedVec2D::new(T::default(), Vector2u::new(1, 1)),
            }
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
