use crate::interval::Interval;
use crate::ray::Point;

#[derive(Debug, Default, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

// impl Hittable for Aabb {
//     fn bounding_box(&self) -> &Aabb {
//         self
//     }
//     fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {

//     }
// }

impl Aabb {
    pub const fn new_with_interval(ix: Interval, iy: Interval, iz: Interval) -> Self {
        Self {
            x: ix,
            y: iy,
            z: iz,
        }
    }
    pub fn new(a: Point, b: Point) -> Self {
        Self::new_with_interval(
            Interval::new(a.x.min(b.x), a.x.max(b.x)),
            Interval::new(a.y.min(b.y), a.y.max(b.y)),
            Interval::new(a.z.min(b.z), a.z.max(b.z)),
        )
    }
    pub fn new_with_boxes(box0: &Self, box1: &Self) -> Self {
        Self {
            x: Interval::new_with_interval(box0.x, box1.x),
            y: Interval::new_with_interval(box0.y, box1.y),
            z: Interval::new_with_interval(box0.z, box1.z),
        }
    }
    pub const fn axis(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
    pub fn hit(&self, r: &crate::ray::Ray, mut ray_t: Interval) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min() - orig) * inv_d;
            let mut t1 = (self.axis(a).max() - orig) * inv_d;

            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }

            if t0 > ray_t.min() {
                ray_t.min = t0;
            }
            if t1 < ray_t.max() {
                ray_t.max = t1;
            }

            if ray_t.max() <= ray_t.min() {
                return false;
            }
        }
        true
    }
}
