pub mod camera;
pub mod color;
pub mod hittables;
pub mod interval;
pub mod materials;
pub mod ray;
pub mod util;
pub mod vec3;

use std::f64::consts::PI;

use camera::Camera;
use color::Color;
use materials::{dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal};
use ray::Point3;

use crate::hittables::{hittable::HittableType, hittable_list::HittableList, sphere::Sphere};

fn main() {
    let mut world: HittableList = HittableList::default();

    let r = (PI / 4f64).cos();
    let material_left: Material =
        Material::Lambertian(Lambertian::new(Color::new(0f64, 0f64, 1f64)));
    let material_right: Material =
        Material::Lambertian(Lambertian::new(Color::new(1f64, 0f64, 0f64)));

    let sphere_left: HittableType =
        HittableType::Sphere(Sphere::new(&Point3::new(-r, 0f64, -1f64), r, material_left));
    let sphere_right: HittableType =
        HittableType::Sphere(Sphere::new(&Point3::new(r, 0f64, -1f64), r, material_right));
    world.add(&sphere_left);
    world.add(&sphere_right);

    let aspect_ratio: f64 = 16f64 / 9f64;
    let image_width: f64 = 400f64;
    let samples_per_pixel: f64 = 100f64;
    let max_depth: f64 = 50f64;
    let vfov: f64 = 90f64;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
    );

    camera.render(HittableType::List(&world));
}
