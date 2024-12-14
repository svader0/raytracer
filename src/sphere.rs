use std::sync::Arc;

use crate::{
    hit::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    util::Interval,
    vec3::Vec3,
};

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = (-half_b - discriminant.sqrt()) / a;
            if ray_t.surrounds(root) {
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.material = Arc::clone(&self.material);
                return true;
            }
        }
        false
    }
}
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
            material: Arc::clone(&material),
        }
    }
}
