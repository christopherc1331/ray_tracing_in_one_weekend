use super::material::Scatter;
use crate::{color::Color, ray::Ray, vec3::random_unit_vector};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
