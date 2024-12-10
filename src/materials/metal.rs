use crate::{
    color::Color,
    ray::Ray,
    vec3::{reflect, Vec3},
};

use super::material::Scatter;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    fn new(albedo: Color) -> Self {
        Self { albedo }
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
        let reflected: Vec3 = reflect(&r_in.direction(), &rec.normal);
        let scattered: Ray = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
