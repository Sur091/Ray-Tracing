// use log::{info,warn};

mod camera;
use camera::Camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;
mod utility;

use ray::Point;

use crate::{hittable::HittableObject, hittable_list::HittableList, sphere::Sphere};

fn main() {
    // World
    let mut world = HittableList::default();
    world.add(HittableObject::Sphere(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(HittableObject::Sphere(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
    )));
    let mut cam: Camera = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
