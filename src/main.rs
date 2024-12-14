use std::sync::Arc;

use camera::Camera;
use hit::Hittables;
use vec3::{Color, Vec3};

pub mod camera;
pub mod hit;
pub mod material;
pub mod quad;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

fn main() {
    let mut world = Hittables::new();

    // spheres
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.5),
        0.5,
        Arc::new(material::Lambertian::new(Color::new(0.4, 0.4, 0.8))),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(-1.0, 0.0, -1.5),
        0.5,
        Arc::new(material::Dielectric::new(1.00 / 1.33)),
    )));

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(1.0, 0.0, -1.5),
        0.5,
        Arc::new(material::Metal::new(Color::new(0.8, 0.3, 0.8), 0.1)),
    )));

    // floor
    world.add(Box::new(quad::Quad::new(
        Vec3::new(-2.0, -0.5, -2.0),
        Vec3::new(-2.0, -0.5, 1.0),
        Vec3::new(2.0, -0.5, 1.0),
        Vec3::new(2.0, -0.5, -2.0),
        Arc::new(material::Lambertian::new(Color::new(0.8, 0.8, 0.8))),
    )));

    let camera = Camera::new(5.0 / 4.0, 400, 30, 10);

    camera.render(&world);
}
