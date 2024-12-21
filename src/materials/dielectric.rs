use crate::{
    color::Color,
    ray::Ray,
    vec3::{refract, unit_vector, Vec3},
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
        let refracted: Vec3 = refract(&unit_direction, &rec.normal, ri);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}
