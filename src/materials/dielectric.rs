use crate::{
    color::Color,
    ray::Ray,
    vec3::{dot, reflect, refract, unit_vector, Vec3},
};

use super::material::Scatter;

#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittables::hittable::HitRecord,
        attenuation: &mut crate::color::Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        *attenuation = Color::new(1f64, 1f64, 1f64);
        let ri: f64 = match rec.front_face {
            true => 1f64 / self.refraction_index,
            false => self.refraction_index,
        };
        let unit_direction: Vec3 = unit_vector(r_in.direction());
        let cos_theta: f64 = dot(-unit_direction, rec.normal).min(1f64);
        let sin_theta: f64 = (1f64 - cos_theta.powi(2)).sqrt();
        let cannot_refract = ri * sin_theta > 1f64;
        let direction: Vec3 = match cannot_refract {
            true => reflect(&unit_direction, &rec.normal),
            false => refract(&unit_direction, &rec.normal, ri),
        };
        *scattered = Ray::new(rec.p, direction);
        true
    }
}
