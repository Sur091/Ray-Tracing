use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::{Direction, Point, Ray};

#[derive(Debug, Clone)]
pub struct Sphere {
    center1: Point,
    radius: f32,
    mat: Material,
    is_moving: bool,
    center_vec: Direction,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center1: Point, center2: Point, radius: f32, mat: Material, bbox: Aabb) -> Self {
        let center_vec = center2 - center1;
        Self {
            center1,
            radius,
            mat,
            center_vec,
            is_moving: !center_vec.abs_diff_eq(Point::new(0.0, 0.0, 0.0), 1e-8),
            bbox,
        }
    }
    fn center(&self, time: f32) -> Point {
        self.center1 + self.center_vec * time
    }

    fn get_sphere_uv(p: &Point, u: &mut f32, v: &mut f32) {
        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + std::f32::consts::PI;

        *u = phi / std::f32::consts::TAU;
        *v = theta / std::f32::consts::PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.center(ray.time())
        } else {
            self.center1
        };
        let oc = ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        // let c = oc.length_squared() - self.radius * self.radius;
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        // let discriminant = half_b * half_b - a * c;
        let discriminant = half_b.mul_add(half_b, -a * c);
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable region
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        Self::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        true
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
