use crate::{interval::Interval, vec3::Vec3};
use std::io::Write;

pub type Color = Vec3;

static INTENSITY: Interval = Interval::new(0f64, 0.999f64);
pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    // Translate the [0, 1] component values to the byte range [0, 25]
    let r_byte: i64 = (255.999f64 * INTENSITY.clamp(r)) as i64;
    let g_byte: i64 = (255.999f64 * INTENSITY.clamp(g)) as i64;
    let b_byte: i64 = (255.999f64 * INTENSITY.clamp(b)) as i64;

    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte).expect("to write ppm metadata");
}
