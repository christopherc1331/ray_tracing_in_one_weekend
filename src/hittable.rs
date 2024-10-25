use crate::{
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    fn set_face_normal(mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.normal = match dot(*r.direction(), *outward_normal) < 0f64 {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }
}
