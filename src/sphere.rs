use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::{Direction, Point, Ray};

#[derive(Debug, Clone)]
pub struct Sphere {
    center1: Point,
    radius: f64,
    mat: Material,
    is_moving: bool,
    center_vec: Direction,
}

impl Sphere {
    pub fn new(center1: Point, center2: Point, radius: f64, mat: Material) -> Self {
        let center_vec = center2-center1;
        Self {
            center1,
            radius,
            mat,
            center_vec,
            is_moving: !center_vec.near_zero(),
        }
    }
    fn center(&self, time: f64) -> Point {
        return  self.center1 + self.center_vec * time;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {self.center(ray.time())} else {self.center1};
        let oc = ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable region
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_t.min() || ray_t.max() <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_t.min() || ray_t.max() <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
