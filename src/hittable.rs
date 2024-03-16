use crate::interval::Interval;
use crate::material::Material;
use crate::ray::{Direction, Point, Ray};
use crate::sphere::Sphere;

pub enum HittableObject {
    Sphere(Sphere),
}

impl HittableObject {
    pub fn sphere(center: Point, radius: f64, mat: Material) -> Self {
        Self::Sphere(Sphere::new(center, radius, mat))
    }
}

impl Hittable for HittableObject {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            HittableObject::Sphere(sphere) => sphere.hit(r, ray_t, rec),
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Debug, Default, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Direction,
    pub mat: Material,
    pub t: f64,
    pub front_face: bool,
}

// impl Material for HitRecord {

// }

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Direction) {
        // Sets the hit record normal vector
        // NOTE: parameter outward_normal is assumed to have unit length
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            (*outward_normal) * (-1.0)
        }
    }
}
