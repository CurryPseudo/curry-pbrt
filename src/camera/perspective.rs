use super::Camera;
use crate::{
    def::Float,
    geometry::{
        Bounds2f, Point2f, Point3f, Ray, Transform, Transformable, Vector2f, Vector2i, Vector3f, Vector2u,
    },
};

pub struct PerspectiveCamera {
    raster_to_camera: Transform,
}

impl PerspectiveCamera {
    pub fn new(fov: Float, resolution: Vector2u) -> Self {
        let near = 1e-2;
        let far = 1000.;
        let resolution = Vector2f::new(resolution.x as Float, resolution.y as Float);
        let aspect = resolution.x / resolution.y;
        let screen_window = if aspect > 1. {
            Bounds2f::new(Point2f::new(-aspect, -1.), Point2f::new(-aspect, 1.))
        } else {
            Bounds2f::new(
                Point2f::new(-1., -1. / aspect),
                Point2f::new(1., 1. / aspect),
            )
        };
        let screen_window_d = screen_window.diagonal();
        let screen_to_raster = Transform::translate(Vector3f::new(
            -screen_window.min.x,
            -screen_window.min.y,
            1.,
        ))
        .apply(&Transform::scale(Vector3f::new(
            1. / screen_window_d.x,
            1. / screen_window_d.y,
            1.,
        )))
        .apply(&Transform::scale(Vector3f::new(
            resolution.x,
            resolution.y,
            1.,
        )));
        let camera_to_screen = Transform::perspective(fov, near, far);
        let camera_to_raster = camera_to_screen.apply(&screen_to_raster);
        Self {
            raster_to_camera: camera_to_raster.inverse(),
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, film: Point2f) -> Ray {
        let film = Point3f::new(film.x, film.y, 0.);
        let camera = film.apply(&self.raster_to_camera);
        Ray::new_od(Point3f::new(0., 0., 0.), camera.coords.normalize())
    }
}
