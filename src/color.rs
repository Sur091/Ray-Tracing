use crate::interval::Interval;
use crate::vec3;
pub type Color = vec3::Vec3<f64>;




pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    //Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    // Write th translated [0,255] value  of each color component
    let intensity = Interval::new(0.000, 0.999);
    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    );
}
