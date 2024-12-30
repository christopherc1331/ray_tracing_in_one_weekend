use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use rayon::prelude::*; // Ensure rayon's parallel iterator traits are imported

use crate::{
    color::{build_color, write_color, Color},
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
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render(self, world: HittableType) {
        let render_sw = SystemTime::now();
        let mut stdout = io::stdout();
        let file = std::fs::File::create("image.ppm").expect("Image file to be created");
        let mut buff = std::io::BufWriter::new(file);
        writeln!(buff, "P3\n").expect("to write ppm metadata");
        writeln!(buff, "{} {}\n", self.image_width, self.image_height)
            .expect("to write ppm metadata");
        writeln!(buff, "255\n").expect("to write ppm metadata");

        let default_value: [u8; 11] = [b' '; 11];
        let colors = Mutex::new(vec![
            default_value;
            (self.image_height * self.image_width) as usize
        ]);
        let colors_len: &usize = &colors.lock().unwrap().len();
        let completed_ct: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));

        // Parallelize the outer loop
        (0..(self.image_height as i16))
            .into_par_iter() // Convert into a parallel iterator
            .for_each(|j| {
                for i in 0..(self.image_width as i16) {
                    let mut pixel_color: Color = Color::default();
                    for _ in 0..self.samples_per_pixel as i64 {
                        let ray: Ray = self.get_ray(i as f64, j as f64);
                        pixel_color += Ray::ray_color(&ray, self.max_depth, &world);
                    }
                    let color: [u8; 11] = build_color(self.pixel_samples_scale * pixel_color);
                    let mut colors_guard = colors.lock().unwrap();
                    let idx = (j as usize * self.image_width as usize) + i as usize;

                    colors_guard[idx] = color;
                    let mut count_guard = completed_ct.lock().unwrap();
                    *count_guard += 1;
                    print!(
                        "\r|Render| Progress: {:.3}%",
                        (*count_guard as f32 / (*colors_len - 1) as f32) * 100.0
                    );
                }
            });

        stdout.flush().unwrap();
        println!();
        println!(
            "Render time (minutes): {}",
            render_sw.elapsed().unwrap().as_secs() as f32 / 60.0
        );

        stdout.flush().unwrap();
        println!();

        // Write all the colors to the file
        let write_sw = SystemTime::now();
        let color_len = colors.lock().unwrap().len();
        for (i, color) in colors.lock().unwrap().iter().enumerate() {
            print!(
                "\r|Write| Progress: {:.3}%",
                (i as f32 / (color_len - 1) as f32) * 100.0
            );
            write_color(&mut buff, u8_to_string(color));
        }

        stdout.flush().unwrap();
        println!();
        println!(
            "Write time (minutes): {}",
            write_sw.elapsed().unwrap().as_secs() as f32 / 60.0
        );
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

fn u8_to_string(bytes: &[u8; 11]) -> String {
    std::str::from_utf8(bytes)
        .expect("Invalid UTF-8")
        .to_string()
}

pub fn string_to_u8(string: &str) -> [u8; 11] {
    let mut array = [b' '; 11]; // Initialize with spaces
    let bytes = string.as_bytes();
    let len = bytes.len().min(11); // Truncate if necessary
    array[..len].copy_from_slice(&bytes[..len]);
    array
}
