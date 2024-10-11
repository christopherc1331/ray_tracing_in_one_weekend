pub mod vec3;

use std::{
    fs,
    io::{self, Write},
};
use vec3::Vec3;

fn main() {
    let image_width: i16 = 256;
    let image_height: i16 = 256;
    let mut lines: Vec<String> = Vec::new();

    lines.push("P3\n".into());
    lines.push(format!("{} {}\n", image_width, image_height));
    lines.push("255\n".into());

    let mut stdout = io::stdout();
    for j in 0..image_height {
        print!("\rScanlines remaining: {}", image_height - j);
        stdout.flush().unwrap();
        for i in 0..image_width {
            let r: f32 = i as f32 / (image_width as f32 - 1f32);
            let g: f32 = j as f32 / (image_height as f32 - 1f32);
            let b: f32 = 0f32;
            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;
            lines.push(format!("{} {} {}", ir, ig, ib));
        }
    }

    println!();
    println!("Done.");
    let joined = lines.join("\n");
    let _ = fs::write("image.ppm", joined);
}
