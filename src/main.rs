pub mod color;
pub mod ray;
pub mod vec3;

use color::Color;
use std::io::{self, Write};
use vec3::Vec3;

use crate::color::write_color;

fn main() {
    let image_width: i16 = 256;
    let image_height: i16 = 256;

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
