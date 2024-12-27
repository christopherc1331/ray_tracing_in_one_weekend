use std::{borrow::BorrowMut, rc::Rc};

use super::hittable::{HitRecord, Hittable, HittableType};
use crate::interval::Interval;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<HittableType>>,
}

impl HittableList {
    pub fn new(object: HittableType) -> Self {
        Self {
            objects: vec![Rc::new(object)],
        }
    }

    pub fn add(&mut self, object: HittableType) {
        self.objects.push(Rc::new(object));
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
                _ => false,
            } {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
