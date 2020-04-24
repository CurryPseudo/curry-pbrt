use crate::*;
use std::fs::File;
use std::path::Path;

pub trait ImageTextureContent {
    fn default() -> Self;
    fn from_rgb_spectrum(s: RGBSpectrum) -> Self;
}

impl ImageTextureContent for Float {
    fn default() -> Self {
        0.
    }
    fn from_rgb_spectrum(s: RGBSpectrum) -> Self {
        s.y()
    }
}

impl ImageTextureContent for Spectrum {
    fn default() -> Self {
        Spectrum::new(0.)
    }
    fn from_rgb_spectrum(s: RGBSpectrum) -> Self {
        s
    }
}

#[derive(Debug)]
pub struct ImageTexture<T> {
    pixels: FixedVec2D<T>,
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
                    rgb.push(inverse_gamma_correct(buf[i * 3 + j] as Float / 255.));
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

impl<T: Clone + std::marker::Sync + std::marker::Send + std::fmt::Debug> Texture<T>
    for ImageTexture<T>
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
}
