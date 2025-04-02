pub mod camera;
pub mod color;
pub mod hittables;
pub mod interval;
pub mod materials;
pub mod ray;
pub mod util;
pub mod vec3;

use camera::{Camera, CameraConfig};
use color::Color;
use materials::{dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal};
use ray::Point3;
use util::{random_double, random_double_range};
use vec3::Vec3;

use crate::hittables::{hittable::HittableType, hittable_list::HittableList, sphere::Sphere};

use std::sync::Arc;

fn main() {
    let mut world: HittableList = HittableList::default();

    let ground_material: Material =
        Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let sphere_ground: HittableType = HittableType::Sphere(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    world.add(sphere_ground);

    let fixed_point: &Point3 = &Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let rand_double: f64 = random_double();
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - *fixed_point).length() <= 0.9 {
                continue;
            }

            let sphere: HittableType = match rand_double {
                rd if rd < 0.8 => {
                    // diffuse
                    let albedo: Color = Color::random() * Color::random();
                    let material: Material = Material::Lambertian(Lambertian::new(albedo));
                    HittableType::Sphere(Arc::new(Sphere::new(&center, 0.2, material)))
                }
                rd if rd < 0.95 => {
                    // metal
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = random_double_range(0.0, 0.5);
                    let material: Material = Material::Metal(Metal::new(albedo, fuzz));
                    HittableType::Sphere(Arc::new(Sphere::new(&center, 0.2, material)))
                }
                _ => {
                    // glass
                    let material: Material = Material::Dielectric(Dielectric::new(1.5));
                    HittableType::Sphere(Arc::new(Sphere::new(&center, 0.2, material)))
                }
            };

            world.add(sphere);
        }
    }

    let material1: Material = Material::Dielectric(Dielectric::new(1.5));
    world.add(HittableType::Sphere(Arc::new(Sphere::new(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));

    let material2: Material = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(HittableType::Sphere(Arc::new(Sphere::new(
        &Color::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ))));

    let material3: Material = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(HittableType::Sphere(Arc::new(Sphere::new(
        &Color::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ))));

    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200.0,
        samples_per_pixel: 500.0,
        max_depth: 50,
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Vec3::new(0.0, 0.2, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    });

    camera.render(HittableType::List(Arc::new(world)));
}
