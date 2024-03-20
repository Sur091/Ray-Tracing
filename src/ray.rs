#![allow(dead_code)]

use glam::Vec3;

pub type Point = Vec3;
pub type Direction = Vec3;

#[derive(Debug, Default)]
pub struct Ray {
    origin: Point,
    direction: Direction,
    time: f32,
}

impl Ray {
    pub const fn new(origin: Point, direction: Direction, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub const fn origin(&self) -> Point {
        self.origin
    }
    pub const fn direction(&self) -> Direction {
        self.direction
    }
    pub const fn time(&self) -> f32 {
        self.time
    }
    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
