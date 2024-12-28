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
use std::sync::{Arc, Mutex}; // Import Arc and Mutex for interior mutability
use util::{random_double, random_double_range};
use vec3::Vec3;

use crate::hittables::{hittable::HittableType, hittable_list::HittableList, sphere::Sphere};

fn main() {
    // Wrap HittableList in Arc and Mutex to allow mutable access
    let world: Arc<Mutex<HittableList>> = Arc::new(Mutex::new(HittableList::default()));

    let ground_material = Arc::new(Material::Lambertian(Lambertian::new(Color::new(
        0.5, 0.5, 0.5,
    ))));
    let sphere_ground = HittableType::Sphere(Arc::new(Sphere::new(
        &Point3::new(0.0, -1000.5, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    // Lock the Mutex to modify the world
    {
        let mut world = world.lock().unwrap();
        world.add(sphere_ground);
    }

    let fixed_point: &Point3 = &Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let rand_double: f64 = random_double();
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.0,
                b as f64 + 0.9 * random_double(),
            );
            if (center - *fixed_point).length() <= 0.9 {
                continue;
            }

            match rand_double {
                rd if rd < 0.8 => {
                    // diffuse
                    let albedo: Color = Color::random() * Color::random();
                    let material: Arc<Material> =
                        Arc::new(Material::Lambertian(Lambertian::new(albedo)));
                    let sphere: HittableType =
                        HittableType::Sphere(Arc::new(Sphere::new(&center, 0.2, material.clone())));

                    // Lock the Mutex to modify the world
                    let mut world = world.lock().unwrap();
                    world.add(sphere);
                }
                rd if rd < 0.95 => {
                    // metal
                    let albedo: Color = Color::random_range(0.5, 1.0);
                    let fuzz: f64 = random_double_range(0.0, 0.5);
                    let material: Arc<Material> =
                        Arc::new(Material::Metal(Metal::new(albedo, fuzz)));
                    let sphere: HittableType =
                        HittableType::Sphere(Arc::new(Sphere::new(&center, 0.2, material)));

                    // Lock the Mutex to modify the world
                    let mut world = world.lock().unwrap();
                    world.add(sphere);
                }
                _ => {
                    // glass
                    let material: Arc<Material> =
                        Arc::new(Material::Dielectric(Dielectric::new(1.5)));
                    let sphere: HittableType =
                        HittableType::Sphere(Arc::new(Sphere::new(&center, 0.2, material)));

                    // Lock the Mutex to modify the world
                    let mut world = world.lock().unwrap();
                    world.add(sphere);
                }
            }
        }
    }

    let material1: Arc<Material> = Arc::new(Material::Dielectric(Dielectric::new(1.5)));
    {
        let mut world = world.lock().unwrap();
        world.add(HittableType::Sphere(Arc::new(Sphere::new(
            &Point3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        ))));
    }

    let material2: Arc<Material> = Arc::new(Material::Lambertian(Lambertian::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    {
        let mut world = world.lock().unwrap();
        world.add(HittableType::Sphere(Arc::new(Sphere::new(
            &Color::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        ))));
    }

    let material3: Arc<Material> =
        Arc::new(Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    {
        let mut world = world.lock().unwrap();
        world.add(HittableType::Sphere(Arc::new(Sphere::new(
            &Color::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        ))));
    }

    let camera = Camera::new(CameraConfig {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200.0,
        samples_per_pixel: 500.0,
        max_depth: 50,
        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    });

    // Lock the Mutex to render
    camera.render(HittableType::List(Arc::new(
        world.lock().unwrap().clone_objects(),
    )));
}
