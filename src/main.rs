use std::{io::stdout, time::Duration};

use camera::Camera;
use hit::Hittables;
use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use vec3::Vec3;

pub mod camera;
pub mod hit;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

fn main() {
    let mut world = Hittables::new();

    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let camera = Camera::new(16.0 / 9.0, 400, 10);

    camera.render(&world);
}
