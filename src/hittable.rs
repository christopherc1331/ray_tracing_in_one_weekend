use crate::{
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    ray::{Point3, Ray},
    sphere::Sphere,
    vec3::{dot, Vec3},
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Material,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.normal = match dot(r.direction(), *outward_normal) < 0f64 {
            true => *outward_normal,
            false => -*outward_normal,
        };
    }
}

pub enum HittableType<'a> {
    Sphere(Sphere),
    List(&'a HittableList<'a>),
}

pub trait Hittable<'a> {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> bool;
}

impl<'a> HittableType<'a> {
    pub fn hit(&self, r: &Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> bool {
        match self {
            Self::List(l) => l.hit(r, ray_t, rec),
            Self::Sphere(s) => s.hit(r, ray_t, rec),
        }
    }
}
