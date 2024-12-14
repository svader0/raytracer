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

#[inline(always)]
pub fn random_float_range(min: f64, max: f64) -> f64 {
    // Returns a random float in [min,max)
    return min + (max - min) * rand::random::<f64>();
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, value: f64) -> bool {
        return self.min <= value && value <= self.max;
    }

    pub fn surrounds(&self, value: f64) -> bool {
        return self.min < value && value < self.max;
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min {
            return self.min;
        } else if value > self.max {
            return self.max;
        } else {
            return value;
        }
    }
}

const EMPTY_INTERVAL: Interval = Interval {
    min: infinty,
    max: -infinty,
};

const UNIVERSE_INTERVAL: Interval = Interval {
    min: -infinty,
    max: infinty,
};
