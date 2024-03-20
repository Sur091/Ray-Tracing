use glam::Vec3;

use crate::interval::Interval;
pub type Color = Vec3;

fn linear_to_gamma(linear_component: f32) -> f32 {
    linear_component.sqrt()
}

pub fn write_color(pixel_color: Color, samples_per_pixel: u16) {
    //Divide the color by the number of samples.
    let scale = 1.0 / f32::from(samples_per_pixel);

    let r = linear_to_gamma(pixel_color.x * scale);
    let g = linear_to_gamma(pixel_color.y * scale);
    let b = linear_to_gamma(pixel_color.z * scale);

    // Write th translated [0,255] value  of each color component
    let intensity = Interval::new(0.000, 0.999);
    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i16,
        (256.0 * intensity.clamp(g)) as i16,
        (256.0 * intensity.clamp(b)) as i16
    );
}
