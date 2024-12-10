use std::f64::INFINITY;

use super::hittables::hittable::{HitRecord, Hittable, HittableType};
use crate::{
    color::Color,
    interval::Interval,
    vec3::{dot, random_on_hemisphere, random_unit_vector, unit_vector, Vec3},
};

pub type Point3 = Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + (t * self.dir)
    }

    // lerp function: blendedValue = (1 − a) * startValue + a * endValue,
    pub fn ray_color(&self, depth: f64, world: &HittableType) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0f64 {
            return Color::default();
        }

        let mut rec: HitRecord = HitRecord::default();
        if world.hit(self, &Interval::new(0.001f64, INFINITY), &mut rec) {
            let direction = rec.normal + random_unit_vector();
            let random_ray = Ray::new(rec.p, direction);
            return 0.5f64 * random_ray.ray_color(depth - 1f64, world);
        }
        let unit_direction: Vec3 = unit_vector(self.direction());
        let a = 0.5 * (unit_direction.y() + 1f64);
        (1f64 - a) * Color::new(1f64, 1f64, 1f64) + (a * Color::new(0.5f64, 0.7f64, 1.0f64))
    }
}
