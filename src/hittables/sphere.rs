use std::rc::Rc;

use super::hittable::{HitRecord, Hittable};
use crate::{
    interval::Interval,
    materials::material::Material,
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: Material) -> Self {
        Self {
            center: *center,
            radius: radius.max(0f64),
            mat: Rc::new(mat),
        }
    }
}

impl<'a> Hittable<'a> for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = dot(r.direction(), oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = h * h - a * c;
        if discriminant < 0f64 {
            return false;
        }
        let sqrtd: f64 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.mat = self.mat.as_ref().clone();
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
