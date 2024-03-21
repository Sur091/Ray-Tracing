use super::HittableObject;
use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::interval::Interval;
use crate::random;

#[derive(Clone)]
pub struct BvhNode {
    left: Box<HittableObject>,
    right: Box<HittableObject>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(mut list: HittableList) -> Self {
        return Self::new_from_vector(list.objects());
    }
    pub fn new_from_vector(src_objects: &mut [HittableObject]) -> Self {
        // let mut objects = src_objects;

        let axis_index = random::number(0, 3);
        let comparator = |a: &HittableObject, b: &HittableObject| {
            return a
                .bounding_box()
                .axis(axis_index)
                .min
                .partial_cmp(&b.bounding_box().axis(axis_index).min)
                .unwrap();
        };

        let (left, right) = if src_objects.len() == 1 {
            (src_objects[0].clone(), src_objects[0].clone())
        } else if src_objects.len() == 2 {
            if comparator(&src_objects[0], &src_objects[1]) == std::cmp::Ordering::Less {
                (src_objects[0].clone(), src_objects[1].clone())
            } else {
                (src_objects[1].clone(), src_objects[0].clone())
            }
        } else {
            src_objects.sort_unstable_by(comparator);
            let mid = src_objects.len() / 2;

            (
                HittableObject::bvh_node(&mut src_objects[..mid]),
                HittableObject::bvh_node(&mut src_objects[mid..]),
            )
        };

        let bbox = Aabb::new_with_boxes(left.bounding_box(), right.bounding_box());
        Self {
            left: Box::new(left),
            right: Box::new(right),
            bbox,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut HitRecord,
    ) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );
        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
