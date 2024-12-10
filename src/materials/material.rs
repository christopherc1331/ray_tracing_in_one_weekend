use crate::{color::Color, hittables::hittable::HitRecord, ray::Ray};

use super::metal::Metal;

#[derive(Clone)]
pub enum Material {
    Metal(Metal),
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
