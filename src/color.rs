use crate::{interval::Interval, vec3::Vec3};
use std::io::Write;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    match linear_component {
        x if x > 0.0 => linear_component.sqrt(),
        _ => 0.0,
    }
}

static INTENSITY: Interval = Interval::new(0.0, 0.999);
pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    // Apply a linear to gamma transform for gamma 2
    let r: f64 = linear_to_gamma(pixel_color.x());
    let g: f64 = linear_to_gamma(pixel_color.y());
    let b: f64 = linear_to_gamma(pixel_color.z());

    // Translate the [0, 1] component values to the byte range [0, 25]
    let r_byte: i64 = (255.999 * INTENSITY.clamp(r)) as i64;
    let g_byte: i64 = (255.999 * INTENSITY.clamp(g)) as i64;
    let b_byte: i64 = (255.999 * INTENSITY.clamp(b)) as i64;

    writeln!(out, "{} {} {}", r_byte, g_byte, b_byte).expect("to write ppm metadata");
}
