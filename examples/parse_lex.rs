use curry_pbrt::scene_file_parser::lex::parse_lex;
use std::path::Path;
fn main() {
    println!(
        "{:#?}",
        parse_lex(Path::new("scenes/landscape/view-0.pbrt"))
    );
}
