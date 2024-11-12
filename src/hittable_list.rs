use std::borrow::BorrowMut;

use crate::hittable::{HitRecord, Hittable, HittableType};

pub struct HittableList<'a> {
    pub objects: Vec<&'a HittableType<'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new(object: &'a HittableType) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: &'a HittableType) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable<'a> for HittableList<'a> {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        mut rec: &'a mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_rec: Option<&HitRecord>;
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;
        for object in &self.objects {
            hit_anything = match object {
                HittableType::Sphere(s) => s.hit(r, ray_tmin, closest_so_far, &mut temp_rec),
                _ => false,
            };
            if hit_anything {
                closest_so_far = temp_rec.t;
                hit_rec = Some(&temp_rec);
            }
        }
        hit_anything
    }
}
