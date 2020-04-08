use curry_pbrt::scene_file_parser::lex::parse_lex;
use std::path::Path;
use std::env::args;
fn main() {
    let args = args().collect::<Vec<_>>();
    println!(
        "{:#?}",
        parse_lex(Path::new(&args[1]))
    );
}
