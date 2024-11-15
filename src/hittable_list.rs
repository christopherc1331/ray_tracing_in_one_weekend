use std::{borrow::BorrowMut, rc::Rc};

use crate::{
    hittable::{HitRecord, Hittable, HittableType},
    interval::Interval,
};

#[derive(Default)]
pub struct HittableList<'a> {
    pub objects: Vec<Rc<&'a HittableType<'a>>>,
}

impl<'a> HittableList<'a> {
    pub fn new(object: &'a HittableType) -> Self {
        Self {
            objects: vec![Rc::new(object)],
        }
    }

    pub fn add(&mut self, object: &'a HittableType) {
        self.objects.push(Rc::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable<'a> for HittableList<'a> {
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
