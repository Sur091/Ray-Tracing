use super::Texture;
use crate::{color::Color, ray::Point};

#[derive(Debug, Clone)]
pub struct Checker {
    inv_scale: f32,
    even: Box<Texture>,
    odd: Box<Texture>,
}

impl Checker {
    pub fn new(scale: f32, even: Box<Texture>, odd: Box<Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
    pub fn new_using_color(scale: f32, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Box::new(Texture::solid_color(c1)),
            Box::new(Texture::solid_color(c2)),
        )
    }
    pub fn value(&self, u: f32, v: f32, p: &Point) -> Color {
        let x_int = (p.x * self.inv_scale).floor() as i32;
        let y_int = (p.y * self.inv_scale).floor() as i32;
        let z_int = (p.z * self.inv_scale).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
