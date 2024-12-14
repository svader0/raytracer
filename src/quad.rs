use std::sync::Arc;

use crate::{
    hit::{HitRecord, Hittable},
    material::{Lambertian, Material},
    ray::Ray,
    util::Interval,
    vec3::{Color, Vec3},
};

pub struct Quad {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub d: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

impl Quad {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3, material: Arc<dyn Material>) -> Quad {
        let normal = (b - a).cross(c - a).unit_vector();
        Quad {
            a,
            b,
            c,
            d,
            normal,
            material,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.b;
        let edge3 = self.d - self.c;
        let edge4 = self.a - self.d;

        let normal = self.normal;

        let t = (self.a - r.origin).dot(normal) / r.direction.dot(normal);
        if !ray_t.surrounds(t) {
            return false;
        }

        let p = r.at(t);

        let c1 = edge1.cross(p - self.a).dot(normal);
        let c2 = edge2.cross(p - self.b).dot(normal);
        let c3 = edge3.cross(p - self.c).dot(normal);
        let c4 = edge4.cross(p - self.d).dot(normal);

        if c1 >= 0.0 && c2 >= 0.0 && c3 >= 0.0 && c4 >= 0.0 {
            rec.t = t;
            rec.p = p;
            rec.normal = normal;
            rec.set_face_normal(r, normal);
            rec.material = Arc::clone(&self.material);
            return true;
        }

        false
    }
}
