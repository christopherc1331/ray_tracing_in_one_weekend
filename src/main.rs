pub mod color;
pub mod ray;
pub mod vec3;

use color::Color;
use ray::Point3;
use std::io::{self, Write};
use vec3::{make_from_dividing_num, make_from_subtracting_vecs, Vec3};

use crate::color::write_color;

fn main() {
    let aspect_ratio: f32 = 16f32 / 9f32;
    let image_width: i16 = 400;
    // Calculate the image height, and ensure that it's at least 1
    let image_height: i16 = match (image_width as f32) / aspect_ratio {
        n if n < 1f32 => 1,
        n => n.round() as i16,
    };

    // Camera
    let focal_length: f64 = 1f64;
    let viewport_height: f64 = 2f64;
    let viewport_width: f64 = viewport_height * (image_width as f64) / (image_height as f64);
    let camera_center: Point3 = Point3::new(0f64, 0f64, 0f64);

    // Calculate the vectors across the horizontal and  donw the vertical viewport edges
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0f64, 0f64);
    let viewport_v: Vec3 = Vec3::new(0f64, -viewport_height, 0f64);

    // Calculate the vectors across the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u: Vec3 = make_from_dividing_num(viewport_u, image_width as f64);
    let pixel_delta_v: Vec3 = make_from_dividing_num(viewport_v, image_height as f64);

    // Calculate the location of the upper left pixel
    // TODO: refactor all these calcs after setting up proper operator overloading
    //let viewport_upper_left = make_from_subtracting_vecs(
    //    camera_center,
    //    make_from_subtracting_vecs(Vec::new(0f64, 0f64, focal_length)),
    //);

    let file = std::fs::File::create("image.ppm").expect("Image file to be created");
    let mut buff = std::io::BufWriter::new(file);

    writeln!(buff, "P3\n").expect("to write ppm metadata");
    writeln!(buff, "{} {}\n", image_width, image_height).expect("to write ppm metadata");
    writeln!(buff, "255\n").expect("to write ppm metadata");

    let mut stdout = io::stdout();
    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        stdout.flush().unwrap();
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width as f64 - 1f64);
            let g: f64 = j as f64 / (image_height as f64 - 1f64);
            let b: f64 = 0f64;
            let pixel_color: Color = Color::new(r, g, b);
            write_color(&mut buff, pixel_color);
        }
    }

    println!();
    println!("Done.");
}
