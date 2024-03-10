use crate::hittable::{HitRecord, Hittable, HittableObject};

pub struct HittableList {
    objects: Vec<HittableObject>,
}

impl HittableList {
    pub fn new(object: HittableObject) -> Self {
        Self {
            objects: vec![object]
        }
    }
    pub fn add(&mut self, object: HittableObject) {
        self.objects.push(object);
    }
}



impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *rec = temp_record.clone();
            }
        }
        return hit_anything
    }
}