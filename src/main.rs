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
use vec3::Vec3;

use crate::hittables::{hittable::HittableType, hittable_list::HittableList, sphere::Sphere};

fn main() {
    let mut world: HittableList = HittableList::default();

    let material_ground: Material =
        Material::Lambertian(Lambertian::new(Color::new(0.8f64, 0.8f64, 0f64)));
    let material_center: Material =
        Material::Lambertian(Lambertian::new(Color::new(0.1f64, 0.2f64, 0.5f64)));
    let material_left: Material = Material::Dielectric(Dielectric::new(1.5f64));
    let material_bubble: Material = Material::Dielectric(Dielectric::new(1f64 / 1.5f64));
    let material_right: Material =
        Material::Metal(Metal::new(Color::new(0.8f64, 0.6f64, 0.2f64), 1f64));

    let sphere_ground: HittableType = HittableType::Sphere(Sphere::new(
        &Point3::new(0f64, -100.5f64, -1f64),
        100f64,
        material_ground,
    ));
    let sphere_center: HittableType = HittableType::Sphere(Sphere::new(
        &Point3::new(0f64, 0f64, -1.2f64),
        0.5f64,
        material_center,
    ));
    let sphere_left: HittableType = HittableType::Sphere(Sphere::new(
        &Point3::new(-1f64, 0f64, -1f64),
        0.5f64,
        material_left,
    ));
    let sphere_bubble: HittableType = HittableType::Sphere(Sphere::new(
        &Point3::new(-1f64, 0f64, -1f64),
        0.4f64,
        material_bubble,
    ));
    let sphere_right: HittableType = HittableType::Sphere(Sphere::new(
        &Point3::new(1f64, 0f64, -1f64),
        0.5f64,
        material_right,
    ));
    world.add(&sphere_ground);
    world.add(&sphere_center);
    world.add(&sphere_left);
    world.add(&sphere_bubble);
    world.add(&sphere_right);

    let aspect_ratio: f64 = 16f64 / 9f64;
    let image_width: f64 = 400f64;
    let samples_per_pixel: f64 = 100f64;
    let max_depth: i16 = 50;
    let vfov: f64 = 20f64;
    let look_from: Point3 = Point3::new(-2f64, 2f64, 1f64);
    let look_at: Point3 = Point3::new(0f64, 0f64, -1f64);
    let v_up: Vec3 = Vec3::new(0f64, 1f64, 0f64);
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        v_up,
    );

    camera.render(HittableType::List(&world));
}
