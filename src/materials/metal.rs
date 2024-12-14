use crate::{
    color::Color,
    ray::Ray,
    vec3::{reflect, Vec3},
};

use super::material::Scatter;

#[derive(Default, Clone, Debug)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
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
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        // println!("metal - ALBEDO: {:?}", self.albedo);
        true
    }
}
