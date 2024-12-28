use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{
    color::{write_color, Color},
    hittables::hittable::HittableType,
    ray::{Point3, Ray},
    util::{degrees_to_radians, random_double},
    vec3::{cross, random_in_unit_disk, unit_vector, Vec3},
};

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub samples_per_pixel: f64,
    pub max_depth: i16,
    pub vfov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub v_up: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

pub struct Camera {
    pixel_samples_scale: f64,
    image_width: f64,
    samples_per_pixel: f64,
    max_depth: i16,
    image_height: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(self, world: HittableType) {
        let file = std::fs::File::create("image.ppm").expect("Image file to be created");
        let buff = Arc::new(Mutex::new(std::io::BufWriter::new(file)));

        writeln!(buff.lock().unwrap(), "P3\n").expect("to write ppm metadata");
        writeln!(
            buff.lock().unwrap(),
            "{} {}\n",
            self.image_width as i16,
            self.image_height as i16
        )
        .expect("to write ppm metadata");
        writeln!(buff.lock().unwrap(), "255\n").expect("to write ppm metadata");

        let image_height = self.image_height as i16;
        let image_width = self.image_width as i16;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;

        // Wrap self in an Arc for thread-safe sharing
        let camera = Arc::new(self);

        // Share `world` among threads
        let world = Arc::new(world);

        // Divide work into rows
        let mut handles = Vec::new();

        // Track scanlines progress
        let progress = Arc::new(Mutex::new(image_height));

        for thread_id in 0..num_cpus::get() {
            let buff = Arc::clone(&buff);
            let world = Arc::clone(&world);
            let camera = Arc::clone(&camera);
            let progress = Arc::clone(&progress);

            let handle = thread::spawn(move || {
                for j in (thread_id as i16..image_height).step_by(num_cpus::get()) {
                    let mut row_buffer = Vec::new();
                    for i in 0..image_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..samples_per_pixel as i64 {
                            let ray = camera.get_ray(i as f64, j as f64);
                            pixel_color += Ray::ray_color(&ray, max_depth, &world);
                        }
                        row_buffer.push(camera.pixel_samples_scale * pixel_color);
                    }

                    // Write the completed row to the buffer
                    let mut buff = buff.lock().unwrap();
                    for color in row_buffer {
                        write_color(&mut *buff, color);
                    }
                    // Update the progress and print the scanlines remaining
                    let mut progress = progress.lock().unwrap();
                    *progress -= 1;
                    print!("\rScanlines remaining: {}", *progress);
                    io::stdout().flush().unwrap();
                }
            });

            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        println!("\rScanlines remaining: 0   ");
        println!("Done.");
    }

    pub fn new(config: CameraConfig) -> Self {
        // Calculate the image height, and ensure that it's at least 1
        let image_height: f64 = match config.image_width / config.aspect_ratio {
            n if n < 1.0 => 1.0,
            n => n.round(),
        };

        let pixel_samples_scale: f64 = 1.0 / config.samples_per_pixel;

        let center: Point3 = config.look_from;

        // Determine the viewport dimensions.
        let theta: f64 = degrees_to_radians(config.vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * config.focus_dist;
        let viewport_width: f64 = viewport_height * (config.image_width / image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w: Vec3 = unit_vector(config.look_from - config.look_at);
        let u: Vec3 = unit_vector(cross(config.v_up, w));
        let v: Vec3 = cross(w, u);

        // Calculate the vectors across the horizontal and  down the vertical viewport edges
        let viewport_u: Vec3 = viewport_width * u;
        let viewport_v: Vec3 = viewport_height * -v;

        // Calculate the vectors across the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u: Vec3 = viewport_u / config.image_width;
        let pixel_delta_v: Vec3 = viewport_v / image_height;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - (config.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius: f64 =
            config.focus_dist * degrees_to_radians(config.defocus_angle / 2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width: config.image_width,
            max_depth: config.max_depth,
            samples_per_pixel: config.samples_per_pixel,
            defocus_angle: config.defocus_angle,
            pixel_samples_scale,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the origin and directed at
        // randomly sampled point around the pixel location i, j.
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);
        let ray_origin: Vec3 = match self.defocus_angle <= 0.0 {
            true => self.center,
            false => self.defocus_disk_sample(),
        };
        let ray_direction: Vec3 = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p: Vec3 = random_in_unit_disk();
        self.center + (p.e[0] * self.defocus_disk_u) + (p.e[1] * self.defocus_disk_v)
    }
}
