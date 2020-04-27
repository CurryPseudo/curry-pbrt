use crate::*;
use openexr::*;
use std::fs::File;
pub struct ExrImageFileReader {}

impl ImageFileReader for ExrImageFileReader {
    fn read_file(&self, mut file: File) -> (Vector2u, Vec<Spectrum>) {
        // Open the EXR file.
        let mut input_file = InputFile::new(&mut file).unwrap();

        // Get the image dimensions, so we know how large of a buffer to make.
        let (width, height) = input_file.header().data_dimensions();

        // Buffer to read pixel data into.
        let mut pixel_data = vec![(0.0f32, 0.0f32, 0.0f32); (width * height) as usize];

        // New scope because `FrameBuffer` mutably borrows `pixel_data`, so we need
        // it to go out of scope before we can access our `pixel_data` again.
        {
            // Create `FrameBufferMut` that points at our pixel data and describes
            // it as RGB data.
            let mut fb = FrameBufferMut::new(width, height);
            fb.insert_channels(&[("R", 0.0), ("G", 0.0), ("B", 0.0)], &mut pixel_data);

            // Read pixel data from the file.
            input_file.read_pixels(&mut fb).unwrap();
        }

        (
            Vector2u::new(width as usize, height as usize),
            pixel_data
                .into_iter()
                .map(|(r, g, b)| Spectrum::from([r, g, b]))
                .collect(),
        )
    }
}
