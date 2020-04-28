use crate::*;
use std::fs::File;
use std::path::Path;
pub struct PngImageFileReader {}
impl ImageFileReader for PngImageFileReader {
    fn read_file(&self, file_path: &Path) -> (Vector2u, Vec<Spectrum>) {
        let file = File::open(file_path).unwrap();
        let decoder = png::Decoder::new(file);
        let (info, mut reader) = decoder.read_info().unwrap();
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();
        let resolution = Vector2u::new(info.width as usize, info.height as usize);
        let mut s = Vec::new();
        for i in 0..buf.len() / 3 {
            s.push(Spectrum::from([
                to_rgb(buf[i * 3]),
                to_rgb(buf[i * 3 + 1]),
                to_rgb(buf[i * 3 + 2]),
            ]));
        }
        (resolution, s)
    }
}
fn to_rgb(u: u8) -> Float {
    u as Float / 255.
}
