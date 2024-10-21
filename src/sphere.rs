use crate::{
    hittable::{HitRecord, Hittable},
    ray::{Point3, Ray},
    vec3::{dot, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Sphere {
            center: *center,
            radius: radius.max(0f64),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - *r.origin();
        let a: f64 = r.direction().length_squared();
        let h: f64 = dot(*r.direction(), oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = h * h - a * c;
        if discriminant < 0f64 {
            return false;
        }
        let sqrtd: f64 = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root: f64 = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}
