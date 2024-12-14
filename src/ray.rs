use crate::{
    hit::Hittable,
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

    // Given a ray, return its color.
    pub fn color(&self, world: &dyn Hittable) -> Color {
        let mut hit_record = crate::hit::HitRecord::new();
        if world.hit(
            self,
            Interval {
                min: 0.0,
                max: f64::INFINITY,
            },
            &mut hit_record,
        ) {
            let direction = Vec3::random_on_hemisphere(hit_record.normal);
            return 0.5 * Ray::new(hit_record.p, direction).color(world);
        }
        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
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
