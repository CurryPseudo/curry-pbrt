use curry_pbrt::{
    camera::PerspectiveCamera,
    film::Film,
    geometry::{shape_apply, Sphere, Transform, TransformShape, Transformable, Vector2u, Vector3f},
    integrator::DirectLightIntegrator,
    light::PointLight,
    material::MatteMaterial,
    primitive::Primitive,
    render::render,
    sampler::{HaltonSampler},
    scene::Scene,
    spectrum::Spectrum,
    texture::Texture,
};
use std::{path::Path, sync::Mutex};

fn main() {
    pretty_env_logger::init();

    let mut scene = Scene::default();
    let sphere = Primitive::new(
        shape_apply(
            Box::new(Sphere::new(1.)),
            &Transform::translate(Vector3f::new(0., 0., 5.)),
        ),
        Box::new(MatteMaterial::new(Texture::from(Spectrum::from([
            0.3, 0.4, 0.5,
        ])))),
    );

    let point_light = Box::new(
        PointLight::new(Spectrum::from([1., 1., 1.]))
            .apply(&Transform::translate(Vector3f::new(2., 0., 5.))),
    );
    scene.add_light(point_light);
    scene.add_primitive(sphere);
    let resolution = Vector2u::new(1024, 768);
    let film = Film::new(resolution);
    let sampler = Box::new(HaltonSampler::new(1, resolution));
    let integrator = Box::new(DirectLightIntegrator::new());
    let camera = Box::new(PerspectiveCamera::new(40., resolution));
    let film = render(scene, sampler, integrator, film, camera);
    film.write_image(&Path::new("image/test.png"));
}
