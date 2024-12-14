use std::{io::stdout, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    hit::Hittables,
    ray::{self, Ray},
    vec3::{Color, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    image_height: u32,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    pub samples_per_pixel: u32,
    pixel_sample_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Camera {
        let aspect_ratio = aspect_ratio;
        let image_width = image_width;
        let samples_per_pixel = samples_per_pixel;

        // Calculate the image height, and ensure that it's at least 1.
        let mut image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        image_height = image_height.clamp(1, image_height);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            pixel_sample_scale,
        }
    }

    pub fn render(&self, world: &Hittables) {
        let bar = ProgressBar::new(self.image_height as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap(),
        );
        bar.enable_steady_tick(Duration::from_millis(100));

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            bar.inc(1);
            for i in 0..self.image_width {
                let color: &mut Color = &mut Color::new(0.0, 0.0, 0.0);
                for _ in 0..(self.samples_per_pixel as u32) {
                    let r = self.get_ray(i, j);
                    *color += r.color(world);
                }
                *color = *color * self.pixel_sample_scale;
                color.write_color(&mut stdout());
            }
        }
        bar.finish_with_message("Done!");
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - self.camera_center;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn sample_square(&self) -> Vec3 {
        Vec3::new(
            rand::random::<f64>() * self.pixel_sample_scale as f64,
            rand::random::<f64>() * self.pixel_sample_scale as f64,
            0.0,
        )
    }
}
