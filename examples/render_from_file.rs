use std::{path::Path, env::args};
use curry_pbrt::render::render_from_file;
use cpuprofiler::PROFILER;

fn main() {
    PROFILER.lock().unwrap().start("./prof.profile").unwrap();
    pretty_env_logger::init();
    let args: Vec<_> = args().collect();
    let file_path = &args[1];
    render_from_file(&Path::new(file_path));
    PROFILER.lock().unwrap().stop().unwrap();
}
