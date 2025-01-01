use std::sync::Arc;

use super::hittable::{HitRecord, Hittable, HittableType};
use crate::interval::Interval;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<HittableType>>, // Use Arc for thread safety
}

impl HittableList {
    pub fn new(object: HittableType) -> Self {
        Self {
            objects: vec![Arc::new(object)], // Wrap object in Arc
        }
    }

    pub fn add(&mut self, object: HittableType) {
        self.objects.push(Arc::new(object)); // Wrap object in Arc
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable<'a> for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if match object.as_ref() {
                HittableType::Sphere(s) => {
                    s.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec)
                }
                HittableType::List(l) => {
                    l.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec)
                }
            } {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

