use std::f64::INFINITY;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable, HittableType},
    interval::Interval,
    vec3::{dot, unit_vector, Vec3},
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
    pub fn ray_color(&self, world: &HittableType) -> Color {
        let mut rec: HitRecord = HitRecord::default();
        if world.hit(self, &Interval::new(0f64, INFINITY), &mut rec) {
            return 0.5f64 * (rec.normal + Color::new(1f64, 1f64, 1f64));
        }
        let unit_direction: Vec3 = unit_vector(self.direction());
        let a = 0.5 * (unit_direction.y() + 1f64);
        (1f64 - a) * Color::new(1f64, 1f64, 1f64) + (a * Color::new(0.5f64, 0.7f64, 1.0f64))
    }
}
