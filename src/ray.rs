use crate::{
    hit::{HitRecord, Hittable},
    util::Interval,
    vec3::{Color, Vec3},
};

type Point3 = Vec3;

// A ray is defined as a function p, such that p(t) = A + tb
// p(t) is a 3D point along the ray, A is the origin, t is a scalar, and b is the direction
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    // Returns the point along the ray at time/scalar t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn color(&self, world: &dyn Hittable, depth: u32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 1.0, 0.0);
        }

        let mut rec = HitRecord::new();

        // If the ray hits an object, scatter the ray and return the color of the scattered ray
        if world.hit(self, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            if rec
                .material
                .scatter(self, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * scattered.color(world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.length_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
