use std::fmt::Debug;
use std::io::Write;
use std::ops::{Add, Div, Mul, Neg, Sub};

const infinty: f64 = f64::INFINITY;
const pi: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * pi / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    return radians * 180.0 / pi;
}
