use crate::{color::Color, vec3::Vec3};

pub type Point3 = Vec3;

pub struct Ray<'a> {
    orig: &'a Point3,
    dir: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point3, direction: &'a Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(self) -> &'a Point3 {
        self.orig
    }

    pub fn direction(self) -> &'a Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Point3 {
        *self.orig + (t * *self.dir)
    }
}

pub fn ray_color(r: &Ray) -> Color {
    Color::new(0f64, 0f64, 0f64)
}
