use rand::prelude::*;
use std::f64::consts::PI;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180f64
}

pub fn random_double() -> f64 {
    thread_rng().gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
