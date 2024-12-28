use std::sync::Arc;

use super::hittable::{HitRecord, Hittable, HittableType};
use crate::interval::Interval;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<HittableType>>, // Changed from Rc to Arc
}

impl HittableList {
    pub fn new(object: HittableType) -> Self {
        Self {
            objects: vec![Arc::new(object)], // Wrap object in Arc
        }
    }

    pub fn add(&mut self, object: HittableType) {
        self.objects.push(Arc::new(object)); // Wrap new object in Arc
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn clone_objects(&self) -> Self {
        HittableList {
            objects: self
                .objects
                .iter()
                .map(|object| Arc::clone(object))
                .collect(),
        }
    }
}

impl<'a> Hittable<'a> for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &'a mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

// impl Clone for HittableList {
//     fn clone(&self) -> Self {
//         self.objects
//             .iter()
//             .map(|object| Arc::clone(object))
//             .collect()
//     }
// }
