use crate::{color::Color, hittables::hittable::HitRecord, ray::Ray};

use super::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

#[derive(Clone, Debug)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

impl Default for Material {
    fn default() -> Self {
        Material::Metal(Metal::default())
    }
}

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
