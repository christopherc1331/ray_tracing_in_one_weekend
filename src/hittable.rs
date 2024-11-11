use crate::{
    hittable_list::HittableList,
    ray::{Point3, Ray},
    sphere::Sphere,
    vec3::{dot, Vec3},
};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub enum HittableType<'a> {
    Sphere(Sphere),
    List(HittableList<'a>),
}

pub trait Hittable<'a> {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &'a mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.normal = match dot(*r.direction(), *outward_normal) < 0f64 {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }
}
