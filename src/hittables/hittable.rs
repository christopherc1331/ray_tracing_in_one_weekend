use super::{hittable_list::HittableList, sphere::Sphere};
use crate::{
    interval::Interval,
    materials::material::Material,
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};

#[derive(Clone, Debug, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Material,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }
}

pub enum HittableType {
    Sphere(Sphere),
    List(HittableList),
}

pub trait Hittable<'a> {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> bool;
}

impl HittableType {
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match self {
            Self::List(l) => l.hit(r, ray_t, rec),
            Self::Sphere(s) => s.hit(r, ray_t, rec),
        }
    }
}
