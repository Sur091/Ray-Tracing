#![allow(dead_code)]

use crate::vec3;

pub type Point = vec3::Vec3<f64>;
pub type Direction = vec3::Vec3<f64>;

#[derive(Debug, Default)]
pub struct Ray {
    origin: Point,
    direction: Direction,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Direction, time: f64) -> Self {
        Self {
            origin,
            direction,
            time
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }
    pub fn direction(&self) -> Direction {
        self.direction
    }
    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
