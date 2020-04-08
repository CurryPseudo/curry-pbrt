use curry_pbrt::scene_file_parser::read_scene;
use std::path::Path;
fn main() {
    println!(
        "{:#?}",
        read_scene(Path::new("scenes/landscape/view-0.pbrt"))
    );
}
