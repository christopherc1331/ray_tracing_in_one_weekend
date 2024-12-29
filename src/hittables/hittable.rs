use std::sync::Arc;

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
    pub mat: Arc<Material>, // Use Arc to make Material thread-safe
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
    Sphere(Arc<Sphere>),     // Use Arc to make Sphere thread-safe
    List(Arc<HittableList>), // Use Arc for thread-safe HittableList
}

pub trait Hittable<'a>: Send + Sync {
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
