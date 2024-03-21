use crate::{color::Color, ray::Point};

mod checker;
mod solid_color;

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(solid_color::SolidColor),
    Checker(checker::Checker),
}

impl Default for Texture {
    fn default() -> Self {
        Self::solid_color(Color::default())
    }
}

impl Texture {
    pub fn value(&self, u: f32, v: f32, p: &Point) -> Color {
        match self {
            Self::SolidColor(solid_color) => solid_color.value(u, v, p),
            Self::Checker(checker) => checker.value(u, v, p),
        }
    }
    pub const fn solid_color(c: Color) -> Self {
        Self::SolidColor(solid_color::SolidColor::new(c))
    }

    pub fn checker(scale: f32, c1: Color, c2: Color) -> Self {
        Self::Checker(checker::Checker::new_using_color(scale, c1, c2))
    }
}
