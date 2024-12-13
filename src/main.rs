use std::{io::stdout, time::Duration};

use hit::Hittables;
use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use vec3::Vec3;

pub mod hit;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    image_height = image_height.clamp(1, image_height);

    let mut world = Hittables::new();
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera settings
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255", image_width, image_height);

    let bar = ProgressBar::new(image_height as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );
    bar.enable_steady_tick(Duration::from_millis(100));

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, direction);
            let color = ray.color(&world);
            color.write_color(&mut stdout());
        }
    }
    bar.finish_with_message("Done!");
}
