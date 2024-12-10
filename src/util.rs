use std::fmt::Debug;
use std::io::Write;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        if length != 0.0 {
            *self / length
        } else {
            *self
        }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, n: f64) -> Vec3 {
        Vec3::new(self.x + n, self.y + n, self.z + n)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, n: f64) -> Vec3 {
        Vec3::new(self.x - n, self.y - n, self.z - n)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, n: f64) -> Vec3 {
        Vec3::new(self.x * n, self.y * n, self.z * n)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self * vec.x, self * vec.y, self * vec.z)
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, n: f64) -> Vec3 {
        Vec3::new(self.x / n, self.y / n, self.z / n)
    }
}

// Type alias for making geometric code more readable
type Point3 = Vec3;

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn write_color(&self, out: &mut std::io::Stdout) {
        let ir = (255.999 * self.r) as i32;
        let ig = (255.999 * self.g) as i32;
        let ib = (255.999 * self.b) as i32;
        writeln!(out, "{} {} {}", ir, ig, ib).expect("Error writing color to output");
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, n: f64) -> Color {
        Color::new(self.r * n, self.g * n, self.b * n)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color::new(self * color.r, self * color.g, self * color.b)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

// A ray is defined as a function p, such that p(t) = A + tb
// p(t) is a 3D point along the ray, A is the origin, t is a scalar, and b is the direction
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
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
    pub fn color(&self) -> Color {
        if self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5) > 0.0 {
            return Color::new(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color = Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
        color
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
