use rand::{rngs::ThreadRng, Rng};

// use crate::color::Color;
// use crate::hittable::HitRecord;
// use crate::ray::Ray;

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(min..max)
}
