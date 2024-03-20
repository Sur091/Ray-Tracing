use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable, HittableObject};
use crate::interval::Interval;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<HittableObject>,
    bbox: Aabb,
}

impl HittableList {
    #![allow(unused)]
    pub fn new(object: HittableObject) -> Self {
        Self {
            objects: vec![object],
            bbox: Aabb::default(),
        }
    }
    pub fn add(&mut self, object: HittableObject) {
        self.bbox = Aabb::new_with_boxes(self.bounding_box(), object.bounding_box());
        self.objects.push(object);
    }
    pub fn objects(&mut self) -> &mut [HittableObject] {
        &mut self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new(ray_t.min(), closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record.clone();
            }
        }
        hit_anything
    }
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
