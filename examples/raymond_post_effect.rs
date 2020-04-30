use curry_pbrt::*;
use std::env::args;
use std::path::Path;
fn main() {
    let args: Vec<_> = args().collect();
    let image_path = &args[1];
    let save_path = &args[2];
    let image: ImageTexture<Spectrum> = ImageTexture::from_file(&Path::new(image_path));
    fn post_effect(origin: &FixedVec2D<Spectrum>, point: Point2u) -> Spectrum {
        let point_f: Point2f = point.coords.map(|u| u as Float).into();
        let radius = 200.;
        let c = 300.;
        let center_f: Point2f = (origin.size() / 2).map(|u| u as Float).into();
        let d = point_f - center_f;
        let distance = d.magnitude();
        if distance > radius {
            return origin[point];
        }
        let normalize_distance = distance / radius;
        let angle_offset = c * (1. - normalize_distance);
        let angle = d.y.atan2(d.x);
        let next_angle = angle + angle_offset.to_radians();
        let next_point: Point2u = (center_f
            + Vector2f::new(next_angle.cos(), next_angle.sin()) * distance)
            .coords
            .map(|f| f as usize)
            .into();
        origin[next_point]
    }
    let image_after = image.post_effect(post_effect);
    image_after.into_file(&Path::new(save_path));
}
