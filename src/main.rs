pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

use camera::Camera;
use ray::Point3;

use crate::{hittable::HittableType, hittable_list::HittableList, sphere::Sphere};

fn main() {
    let mut world: HittableList = HittableList::default();
    let sphere_1: HittableType =
        HittableType::Sphere(Sphere::new(&Point3::new(0f64, 0f64, -1f64), 0.5f64));
    let sphere_2: HittableType =
        HittableType::Sphere(Sphere::new(&Point3::new(0f64, -100.5f64, -1f64), 100f64));
    world.add(&sphere_2);
    world.add(&sphere_1);

    let aspect_ratio: f64 = 16f64 / 9f64;
    let image_width: f64 = 400f64;
    let samples_per_pixel: f64 = 100f64;
    let max_depth: f64 = 50f64;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    camera.render(HittableType::List(&world));
}
