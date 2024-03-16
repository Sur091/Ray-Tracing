// use log::{info,warn};

mod camera;
use camera::Camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

use color::Color;
use ray::Point;

use crate::hittable::HittableObject;
use crate::hittable_list::HittableList;
use crate::material::Material;

fn main() {
    // World
    let mut world = HittableList::default();

    let r: f64 = (std::f64::consts::PI / 4.0).cos();

    // let material_groud = Material::lambertian(&Color::new(0.8, 0.8, 0.0));
    // let material_center = Material::lambertian(&Color::new(0.1, 0.2, 0.5));
    let material_left = Material::lambertian(&Color::new(0.0,0.0,1.0));
    let material_right = Material::lambertian(&Color::new(1.0,0.0,0.0));

    // world.add(HittableObject::sphere(
    //     Point::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_groud,
    // ));
    // world.add(HittableObject::sphere(
    //     Point::new(0.0, 0.0, -1.0),
    //     0.5,
    //     material_center,
    // ));
    // world.add(HittableObject::sphere(
    //     Point::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left.clone(),
    // ));
    world.add(HittableObject::sphere(
        Point::new(-r, 0.0, -1.0),
        r,
        material_left,
    ));
    world.add(HittableObject::sphere(
        Point::new(r, 0.0, -1.0),
        r,
        material_right,
    ));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;


    cam.vfov = 90.0;

    cam.render(&world);
}
