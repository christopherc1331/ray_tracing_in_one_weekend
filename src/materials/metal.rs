use crate::{
    color::Color,
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, unit_vector, Vec3},
};

use super::material::Scatter;

#[derive(Default, Clone, Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittables::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        let mut reflected: Vec3 = reflect(&r_in.direction(), &rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        dot(scattered.direction(), rec.normal) > 0f64
    }
}
