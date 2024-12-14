use std::io::Write;
use std::ops::*;

use rand::random;

use crate::util::{random_float_range, Interval};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

    pub fn random() -> Vec3 {
        Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.length_squared();
        if lensq > 1.0 && lensq > 1e-6 {
            p / lensq.sqrt()
        } else {
            p
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_unit_vector();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
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

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add<Color> for Vec3 {
    type Output = Color;

    fn add(self, color: Color) -> Color {
        Color::new(self.x + color.r, self.y + color.g, self.z + color.b)
    }
}

#[derive(Debug, Clone, Copy)]
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
        static INTENSITY: Interval = Interval {
            min: 0.0,
            max: 0.999,
        };

        let ir = (INTENSITY.clamp(self.r) * 256.0) as i32;
        let ig = (INTENSITY.clamp(self.g) * 256.0) as i32;
        let ib = (INTENSITY.clamp(self.b) * 256.0) as i32;

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

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color::new(self.r + other.r, self.g + other.g, self.b + other.b);
    }
}
