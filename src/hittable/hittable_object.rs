pub mod bvh_node;
mod sphere;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::{Direction, Point, Ray};

#[derive(Clone)]
pub enum HittableObject {
    Sphere(sphere::Sphere),
    BvhNode(bvh_node::BvhNode),
}

impl HittableObject {
    pub fn sphere(center: Point, radius: f32, mat: Material) -> Self {
        let rvec = Direction::new(radius, radius, radius);
        let bbox = Aabb::new(center - rvec, center + rvec);
        Self::Sphere(sphere::Sphere::new(center, center, radius, mat, bbox))
    }
    pub fn moving_sphere(center1: Point, center2: Point, radius: f32, mat: Material) -> Self {
        let rvec = Direction::new(radius, radius, radius);
        let box1 = Aabb::new(center1 - rvec, center1 + rvec);
        let box2 = Aabb::new(center2 - rvec, center2 + rvec);
        let bbox = Aabb::new_with_boxes(&box1, &box2);
        Self::Sphere(sphere::Sphere::new(center1, center2, radius, mat, bbox))
    }
}

impl Hittable for HittableObject {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            Self::Sphere(sphere) => sphere.hit(r, ray_t, rec),
            Self::BvhNode(bvh_node) => bvh_node.hit(r, ray_t, rec),
            // HittableObject::Cube(cub) => cub.hit(r, ray_t, rec)
        }
    }
    fn bounding_box(&self) -> &Aabb {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(),
            Self::BvhNode(bvh_node) => bvh_node.bounding_box(),
        }
    }
}
