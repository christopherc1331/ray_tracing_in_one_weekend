use crate::{color::Color, hittable::HitRecord, ray::Ray};

#[derive(Default, Clone)]
pub enum Material {
    #[default]
    Metal,
}

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        todo!()
    }
}
