pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3;

use color::Color;
use ray::Point3;
use std::io::{self, Write};
use vec3::Vec3;

use crate::{
    color::write_color,
    hittable::HittableType,
    hittable_list::HittableList,
    ray::{ray_color, Ray},
    sphere::Sphere,
};

fn main() {
    let aspect_ratio: f64 = 16f64 / 9f64;
    let image_width: f64 = 400f64;

    // Calculate the image height, and ensure that it's at least 1
    let image_height: f64 = match image_width / aspect_ratio {
        n if n < 1f64 => 1f64,
        n => n.round(),
    };

    // World
    let mut world: HittableList = HittableList::default();
    let sphere_1: &HittableType =
        &hittable::HittableType::Sphere(Sphere::new(&Point3::new(0f64, 0f64, -1f64), 0.5f64));
    let sphere_2: &HittableType =
        &hittable::HittableType::Sphere(Sphere::new(&Point3::new(0f64, -100.5f64, -1f64), 100f64));
    world.add(sphere_1);
    world.add(sphere_2);

    // Camera
    let focal_length: f64 = 1f64;
    let viewport_height: f64 = 2f64;
    let viewport_width: f64 = viewport_height * (image_width / image_height);
    let camera_center: Point3 = Point3::new(0f64, 0f64, 0f64);

    // Calculate the vectors across the horizontal and  donw the vertical viewport edges
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0f64, 0f64);
    let viewport_v: Vec3 = Vec3::new(0f64, -viewport_height, 0f64);

    // Calculate the vectors across the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u: Vec3 = viewport_u / image_width;
    let pixel_delta_v: Vec3 = viewport_v / image_height;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0f64, 0f64, focal_length) - viewport_u / 2f64 - viewport_v / 2f64;
    let pixel00_loc = viewport_upper_left + 0.5f64 * (pixel_delta_u + pixel_delta_v);

    let file = std::fs::File::create("image.ppm").expect("Image file to be created");
    let mut buff = std::io::BufWriter::new(file);

    writeln!(buff, "P3\n").expect("to write ppm metadata");
    writeln!(buff, "{} {}\n", image_width, image_height).expect("to write ppm metadata");
    writeln!(buff, "255\n").expect("to write ppm metadata");

    let mut stdout = io::stdout();
    for j in 0..(image_height as i16) {
        print!("\rScanlines remaining: {}", (image_height as i16) - j);
        stdout.flush().unwrap();
        for i in 0..(image_width as i16) {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);
            let pixel_color: Color = ray_color(&ray, &hittable::HittableType::List(&world));
            write_color(&mut buff, pixel_color);
        }
    }

    println!();
    println!("Done.");
}
