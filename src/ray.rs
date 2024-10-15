use crate::{
    color::Color,
    vec3::{dot, unit_vector, Vec3},
};

pub type Point3 = Vec3;

#[derive(Clone, Copy)]
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

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc: Vec3 = *center - *ray.origin();
    let a = dot(*ray.direction(), *ray.direction());
    let b = -2f64 * dot(*ray.direction(), oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4f64 * a * c;
    discriminant >= 0f64
}

// lerp function: blendedValue = (1 − a) * startValue + a * endValue,
pub fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0f64, 0f64, -1f64), 0.5f64, ray) {
        return Color::new(1f64, 0f64, 0f64);
    }
    let unit_direction: Vec3 = unit_vector(*ray.direction());
    let a = 0.5 * (unit_direction.y() + 1f64);
    (1f64 - a) * Color::new(1f64, 1f64, 1f64) + (a * Color::new(0.5f64, 0.7f64, 1.0f64))
}
