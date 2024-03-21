use crate::color::Color;
use crate::ray::Point;


#[derive(Debug, Clone)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub const fn new(c: Color) -> Self {
        Self {
            color_value: c
        }
    }

    pub const fn new_from_color(r: f32, g: f32, b: f32) -> Self {
        Self::new(Color::new(r, g, b))
    }

    pub fn value(&self, _u: f32, _v: f32, _p: &Point) -> Color {
        self.color_value
    }
}