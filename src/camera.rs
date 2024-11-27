use std::{
    f64::INFINITY,
    io::{self, Write},
};

use crate::{
    color::{write_color, Color},
    hittable::{HitRecord, HittableType},
    hittable_list::HittableList,
    interval::Interval,
    ray::{Point3, Ray},
    util::random_double,
    vec3::{unit_vector, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub samples_per_pixel: i64,
    pixel_samples_scale: i64,
    image_height: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(self, world: HittableList) {
        let file = std::fs::File::create("image.ppm").expect("Image file to be created");
        let mut buff = std::io::BufWriter::new(file);

        writeln!(buff, "P3\n").expect("to write ppm metadata");
        writeln!(buff, "{} {}\n", self.image_width, self.image_height)
            .expect("to write ppm metadata");
        writeln!(buff, "255\n").expect("to write ppm metadata");

        let mut stdout = io::stdout();
        for j in 0..(self.image_height as i16) {
            print!("\rScanlines remaining: {}", (self.image_height as i16) - j);
            stdout.flush().unwrap();
            for _ in 0..(self.image_width as i16) {
                let pixel_color: Color = Color::new(0f64, 0f64, 0f64);
                for _ in 0..self.samples_per_pixel {
                    let ray: Ray;
                }
                // let pixel_center = self.pixel00_loc
                //     + (i as f64 * self.pixel_delta_u)
                //     + (j as f64 * self.pixel_delta_v);
                // let ray_direction = pixel_center - self.center;
                // let ray = Ray::new(&self.center, &ray_direction);
                // let pixel_color: Color = ray.ray_color(&hittable::HittableType::List(&world));
                write_color(&mut buff, pixel_color);
            }
        }

        println!();
        println!("Done.");
    }

    pub fn new(aspect_ratio: f64, image_width: f64, samples_per_pixel: i64) -> Self {
        // Calculate the image height, and ensure that it's at least 1
        let image_height: f64 = match image_width / aspect_ratio {
            n if n < 1f64 => 1f64,
            n => n.round(),
        };

        let pixel_samples_scale: i64 = 1 / samples_per_pixel;

        // Camera
        let focal_length: f64 = 1f64;
        let viewport_height: f64 = 2f64;
        let viewport_width: f64 = viewport_height * (image_width / image_height);
        let camera_center: Point3 = Point3::new(0f64, 0f64, 0f64);

        // Calculate the vectors across the horizontal and  down the vertical viewport edges
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0f64, 0f64);
        let viewport_v: Vec3 = Vec3::new(0f64, -viewport_height, 0f64);

        // Calculate the vectors across the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u: Vec3 = viewport_u / image_width;
        let pixel_delta_v: Vec3 = viewport_v / image_height;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = camera_center
            - Vec3::new(0f64, 0f64, focal_length)
            - viewport_u / 2f64
            - viewport_v / 2f64;
        let pixel00_loc = viewport_upper_left + 0.5f64 * (pixel_delta_u + pixel_delta_v);

        let center: Point3 = Point3::new(0f64, 0f64, 0f64);
        Self {
            pixel_samples_scale,
            samples_per_pixel,
            aspect_ratio,
            image_height,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(ray: Ray, world: HittableType) -> Color {
        let mut rec: HitRecord = HitRecord::default();
        if world.hit(&ray, &Interval::new(0f64, INFINITY), &mut rec) {
            return 0.5f64 * (rec.normal + Color::new(1f64, 1f64, 1f64));
        }
        let unit_direction: Vec3 = unit_vector(ray.direction());
        let a = 0.5f64 * (unit_direction.y() + 1f64);
        (1f64 - a) * Color::new(1f64, 1f64, 1f64) + Color::new(0.5f64, 0.7f64, 1f64)
    }

    fn get_ray(self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the origin and directed at
        // randomly sampled point around the pixel location i, j.
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);
        let ray_origin: Vec3 = self.center;
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5f64, random_double() - 0.5f64, 0f64)
    }
}
