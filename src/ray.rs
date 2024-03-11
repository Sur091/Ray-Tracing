#![allow(dead_code)]

use crate::vec3;

pub type Point = vec3::Vec3<f64>;
pub type Direction = vec3::Vec3<f64>;

#[derive(Debug, Default)]
pub struct Ray {
    origin: Point,
    direction: Direction,
}

impl Ray {
    pub fn new(origin: &Point, direction: &Direction) -> Self {
        Self {
            origin: Point::new(origin.x(), origin.y(), origin.z()),
            direction: Direction::new(direction.x(), direction.y(), direction.z()),
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
