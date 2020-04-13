use curry_pbrt::{
    camera::PerspectiveCamera,
    film::Film,
    geometry::{Sphere, Transform, TransformShape, Transformable, Vector2u, Vector3f},
    integrator::DirectLightIntegrator,
    light::PointLight,
    material::MatteMaterial,
    primitive::Primitive,
    render::render,
    sampler::{HaltonSampler, SamplerWrapper},
    scene::Scene,
    spectrum::Spectrum,
    texture::Texture,
};
use std::{path::Path, sync::Mutex};

fn main() {
    let mut scene = Scene::default();
    let sphere = Primitive::new(
        Box::new(TransformShape::from(Sphere::new(1.)).apply(&Transform::translate(Vector3f::new(0., 0., 1.)))),
        Box::new(MatteMaterial::new(Texture::from(Spectrum::from([
            0.3, 0.4, 0.5,
        ])))),
    );

    let point_light = Box::new(
        PointLight::new(Spectrum::from([1., 1., 1.]))
            .apply(&Transform::translate(Vector3f::new(2., 0., 1.))),
    );
    scene.add_light(point_light);
    scene.add_primitive(sphere);
    let resolution = Vector2u::new(1024, 768);
    let mut film = Film::new(resolution);
    let halton = Mutex::new(HaltonSampler::new());
    let sampler = SamplerWrapper::new(&halton, 1);
    let integrator = Box::new(DirectLightIntegrator::new());
    let camera = Box::new(PerspectiveCamera::new(40., resolution));
    render(scene, sampler, integrator, &mut film, camera);
    film.write_image(&Path::new("image/test.png"));
}
