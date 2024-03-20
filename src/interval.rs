#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
    pub fn new_with_interval(a: Self, b: Self) -> Self {
        Self {
            min: f32::min(a.min(), b.min()),
            max: f32::max(a.max(), b.max()),
        }
    }
    pub fn contains(self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surround(self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    pub const fn min(self) -> f32 {
        self.min
    }
    pub const fn max(self) -> f32 {
        self.max
    }
    pub fn set_min(&mut self, min: f32) {
        self.min = min;
    }
    pub fn set_max(&mut self, max: f32) {
        self.max = max;
    }
    #[inline]
    pub fn clamp(self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
    pub fn expand(self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}
