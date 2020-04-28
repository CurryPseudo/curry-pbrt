use crate::*;
use exr::prelude::*;
use std::fs::File;
use std::path::Path;
pub struct ExrImageFileReader {}

impl ImageFileReader for ExrImageFileReader {
    fn read_file(&self, file_path: &Path) -> (Vector2u, Vec<Spectrum>) {
        let image = simple::Image::read_from_file(
            file_path,
            read_options::high(), // use multi-core decompression
        )
        .unwrap();

        let layer = &image.layers[0];
        let resolution = {
            let data_size = layer.data_size;
            Vector2u::new(data_size.x(), data_size.y())
        };

        let mut s = vec![Spectrum::new(0.); resolution.x * resolution.y];

        for channel in &layer.channels {
            let samples: Vec<Float> = match channel.samples.clone() {
                simple::Samples::F16(f16_vec) => {
                    f16_vec.into_iter().map(|f| f.to_f32() as Float).collect()
                }
                simple::Samples::F32(f32_vec) => f32_vec.into_iter().map(|f| f as Float).collect(),
                simple::Samples::U32(u32_vec) => {
                    u32_vec.into_iter().map(|u| u as Float).collect()
                }
            };
            let name: String = channel.name.clone().into();
            let c = match name.as_str() {
                "R" => {
                    0
                }
                "G" => {
                    1
                }
                "B" => {
                    2
                }
                _ => panic!()
            };
            for i in 0..samples.len() {
                s[i].as_mut()[c] = samples[i];
            }
            //if let Some(layer_name) = &layer.attributes.name {
            //    println!(
            //        "Channel `{}` of Layer `{}` has an average value of {}",
            //        channel.name, layer_name, average
            //    );
            //} else {
            //    println!(
            //        "Channel `{}` has an average value of {}",
            //        channel.name, average
            //    );
            //}
        }
        (resolution, s)
    }
}
