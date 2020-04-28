use super::Camera;
use crate::*;

#[derive(Clone)]
pub struct PerspectiveCamera {
    raster_to_camera: Transform,
    resolution: Vector2f,
}

impl PerspectiveCamera {
    pub fn new(fov: Float, resolution: Vector2u) -> Self {
        let near = 1e-2;
        let far = 1000.;
        let resolution = Vector2f::new(resolution.x as Float, resolution.y as Float);
        let aspect = resolution.x / resolution.y;
        let screen_window = if aspect > 1. {
            Bounds2f::new(&Point2f::new(-aspect, -1.), &Point2f::new(aspect, 1.))
        } else {
            Bounds2f::new(
                &Point2f::new(-1., -1. / aspect),
                &Point2f::new(1., 1. / aspect),
            )
        };
        let screen_window_d = screen_window.diagonal();
        let screen_to_raster = Transform::translate(Vector3f::new(
            -screen_window.min.x,
            -screen_window.max.y,
            0.,
        ))
        .apply(&Transform::scale(Vector3f::new(
            1. / screen_window_d.x,
            -(1. / screen_window_d.y),
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
            resolution,
        }
    }
}

impl PrimitiveClipper for PerspectiveCamera {
    fn clip(&self, primitive: &Primitive) -> bool {
        if primitive.get_source().get_material().is_some() {
            let bound = primitive.bound();
            let camera_to_raster = self.raster_to_camera.clone().inverse();
            for corner in 0..8 {
                let p = bound[corner];
                let raster = p.apply(&camera_to_raster);
                let clip = 
                    //raster.z > 1.
                    raster.z < 0.
                    ||raster.x >= self.resolution.x
                    || raster.x < 0.
                    || raster.y >= self.resolution.y
                    || raster.y < 0.;
                if !clip {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, film: Point2f) -> Ray {
        let film = Point3f::new(film.x, film.y, 0.);
        let camera = film.apply(&self.raster_to_camera);
        Ray::new_od(Point3f::new(0., 0., 0.), camera.coords.normalize())
    }
    fn as_clipper(&self) -> &dyn PrimitiveClipper {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn perspective_works() {
        let camera_point = Point3f::new(0., 0., 1000.);
        let resolution = Vector2u::new(1024, 768);
        let camera = Box::new(PerspectiveCamera::new(40., resolution));
        let r = camera.clone().raster_to_camera.inverse();
        let film = camera_point.apply(&r);
        assert_eq!(film, Point3f::new(512., 384., 1.));
        assert_eq!(
            camera.generate_ray(film.xy()),
            Ray::new_od(Point3f::new(0., 0., 0.), Vector3f::new(0., 0., 1.))
        );
    }
}
