use super::hittables::hittable::{HitRecord, Hittable, HittableType};
use crate::{
    color::Color,
    hittables::sphere::Sphere,
    interval::Interval,
    materials::{
        material::{Material, Scatter},
        metal::Metal,
    },
    vec3::{dot, random_on_hemisphere, random_unit_vector, unit_vector, Vec3},
};

pub type Point3 = Vec3;

#[derive(Clone, Copy, Default, Debug)]
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
    pub fn ray_color(r: &Ray, depth: f64, world: &HittableType) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0f64 {
            return Color::default();
        }

        let mut rec: HitRecord = HitRecord::default();
        if world.hit(r, &Interval::new(0.001f64, f64::INFINITY), &mut rec) {
            let mut scattered: Ray = Ray::default();
            let mut attenuation: Color = Color::default();
            // println!("rec AFTER HIT: {:?}", rec);
            // println!("scattered - BEFORE: {:?}", scattered);
            // println!("attenuation - BEFORE: {:?}", attenuation);
            let is_scattered: bool = match &rec.mat {
                Material::Metal(m) => m.scatter(r, &rec, &mut attenuation, &mut scattered),
                Material::Lambertian(l) => l.scatter(r, &rec, &mut attenuation, &mut scattered),
                Material::Dielectric(d) => d.scatter(r, &rec, &mut attenuation, &mut scattered),
            };
            // println!("attenuation - AFTER: {:?}", attenuation);
            return match is_scattered {
                true => {
                    let fook = attenuation * Ray::ray_color(&scattered, depth - 1f64, world);
                    // println!("scattered - AFTER: {:?}", scattered);
                    return fook;
                }
                false => Color::default(),
            };
        }
        let unit_direction: Vec3 = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1f64);
        (1f64 - a) * Color::new(1f64, 1f64, 1f64) + (a * Color::new(0.5f64, 0.7f64, 1.0f64))
    }
}
