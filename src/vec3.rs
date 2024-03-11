#![allow(dead_code)]

use core::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{ray::Direction, utility::random_double};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3<T: Copy> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T>
where
    T: Copy
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Sqrt<T>
        + DivAssign
        + Div<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }
    pub fn z(&self) -> T {
        self.z
    }
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&mut self) {
        *self /= self.length();
    }
    pub fn dot(self, other: Self) -> T {
        self.x * other.x() + self.y * other.y() + self.z * other.z()
    }
    pub fn cross(self, other: Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn unit_vector(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
    
}

impl Vec3<f64> {
    pub fn random(min: f64, max: f64) -> Self {
        Self {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max),
        }
    }
    pub fn random_in_unit_shpere() -> Self {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_vector() -> Self {
        return Self::unit_vector(&Self::random_in_unit_shpere());
    }

    pub fn random_on_hemisphere(normal: &Direction) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(*normal) > 0.0 {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0        
        }
    }
}

pub trait Sqrt<T> {
    fn sqrt(&self) -> T;
}

impl Sqrt<f32> for f32 {
    fn sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }
}
impl Sqrt<f64> for f64 {
    fn sqrt(&self) -> f64 {
        f64::sqrt(*self)
    }
}

impl<T> Add for Vec3<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T: Copy + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T: Copy + SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl<T: Copy + Mul<Output = T> + Sub<Output = T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl<T: Copy + DivAssign> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl<T: Copy + std::fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn vec3_addition() {
        let p1 = Vec3::new(1.0, 2.0, 1.0);
        let p2 = Vec3::<f64>::default();

        let p3 = p1 + p2;

        assert_eq!(p3, p1);
    }
    #[test]
    fn vec3_addition_inline() {
        let p1 = Vec3::new(1.0, 2.0, 1.0);
        let mut p2 = Vec3::<f64>::default();

        p2 += p1;

        assert_eq!(p2, p1);
    }
    #[test]
    fn vec3_subtraction() {
        let p1 = Vec3::new(1.0, 2.0, 1.0);
        let p2 = Vec3::<f64>::default();

        let p3 = Vec3::new(1.0, 2.0, 1.0);
        let p4 = p3 - p1;

        assert_eq!(p4, p2);
    }
    #[test]
    fn vec3_subtraction_inline() {
        let p1 = Vec3::new(1.0, 2.0, 1.0);
        let mut p2 = Vec3::<f64>::default();

        p2 -= p1;

        assert_eq!(p2, Vec3::new(-1.0f64, -2.0, -1.0));
    }

    #[test]
    fn vec3_scalar_multiplication() {
        let p1 = Vec3::new(1.1, 1.1, 1.1);

        assert_eq!(p1 * 3.0, Vec3::new(1.1 * 3.0, 1.1 * 3.0, 1.1 * 3.0));
    }
    #[test]
    fn vec3_vector_multiplication() {
        let p1 = Vec3::new(1.1, 2.2, 2.2);
        let p2 = Vec3::new(5.1, 1.2, 3.2);

        assert_eq!(p1 * p2, Vec3::new(4.4, 7.7, -9.9));
    }
    #[test]
    fn vec3_scalar_multiplication_inline() {
        let mut p1 = Vec3::new(1.1, 1.1, 1.1);
        p1 *= 3.0;

        assert_eq!(p1, Vec3::new(1.1 * 3.0, 1.1 * 3.0, 1.1 * 3.0));
    }

    #[test]
    fn vec3_scalar_division() {
        let p1 = Vec3::new(1.1, 1.1, 1.1);

        assert_eq!(p1 / 3.0, Vec3::new(1.1 / 3.0, 1.1 / 3.0, 1.1 / 3.0));
    }
    #[test]
    fn vec3_scalar_division_inline() {
        let mut p1 = Vec3::new(1.1, 1.1, 1.1);
        p1 /= 3.0;

        assert_eq!(p1, Vec3::new(1.1 / 3.0, 1.1 / 3.0, 1.1 / 3.0));
    }
    #[test]
    fn vec3_length_squared() {
        let p1 = Vec3::new(1.1f64, 1.1, 1.1);

        assert_eq!(1.1 * 1.1 + 1.1 * 1.1 + 1.1 * 1.1, p1.length_squared());
    }
    #[test]
    fn vec3_length() {
        let p1 = Vec3::new(1.1f64, 1.1, 1.1);

        assert_eq!((1.1 * 1.1 + 1.1 * 1.1 + 1.1 * 1.1f64).sqrt(), p1.length());
    }

    #[test]
    fn vec3_dot() {
        let p1 = Vec3::new(1.1, 2.2, 2.2);
        let p2 = Vec3::new(5.1, 1.2, 3.2);

        assert_eq!(p1.dot(p2), 1.1 * 5.1 + 2.2 * 1.2 + 2.2 * 3.2);
    }
    #[test]
    fn vec3_cross() {
        let p1 = Vec3::new(1.1, 2.2, 2.2);
        let p2 = Vec3::new(5.1, 1.2, 3.2);

        assert_eq!(p1.cross(p2), Vec3::new(4.4, 7.7, -9.9));
    }
}
