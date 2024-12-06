use crate::{color::Color, hittable::HitRecord, ray::Ray};

#[derive(Default, Clone)]
pub enum Material {
    #[default]
    Metal,
}

pub trait Scatter {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool {
        todo!()
    }
}
