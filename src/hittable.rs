mod hit_record;
mod hittable_list;
mod hittable_object;

pub use hit_record::HitRecord;
pub use hittable_list::HittableList;
pub use hittable_object::bvh_node::BvhNode;
pub use hittable_object::HittableObject;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut hit_record::HitRecord) -> bool;
    fn bounding_box(&self) -> &Aabb;
}
