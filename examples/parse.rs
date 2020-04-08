use curry_pbrt::scene_file_parser::read_scene;
use std::path::Path;
use std::env::args;
fn main() {
    let args = args().collect::<Vec<_>>();
    println!(
        "{:#?}",
        read_scene(Path::new(&args[1]))
    );
}
