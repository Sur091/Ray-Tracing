#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo
)]
mod aabb;
mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod random;
mod ray;
mod texture;
mod utility;
mod vec3;

use std::env;

use camera::Camera;
use color::Color;
use hittable::{HittableList, HittableObject};
use material::Material;
use rand::random;
use ray::{Direction, Point};
use texture::Texture;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{} {}", args[0], args[1]);
    let options = if args.len() > 1 {
        if &args[1] == "0" {
            Scene::RandomSpheres
        } else {
            Scene::TwoSpheres
        }
    } else {
        Scene::TwoSpheres
    };

    enum Scene {
        TwoSpheres,
        RandomSpheres,
    }

    match options {
        Scene::RandomSpheres => random_spheres(),
        Scene::TwoSpheres => two_spheres(),
    }
}
fn random_spheres() {
    // World
    let mut world = HittableList::default();

    let checker = Texture::checker(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world.add(HittableObject::sphere(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::lambertian_with_texture(checker),
    ));

    // let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
    // world.add(HittableObject::sphere(
    //     Point::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     ground_material,
    // ));

    for a in -11..11i8 {
        for b in -11..11i8 {
            let choose_mat = random::number(0.0, 1.0);
            let center = Point::new(
                0.9f32.mul_add(random::number(0.0, 1.0), f32::from(a)),
                0.2,
                0.9f32.mul_add(random::number(0.0, 1.0), f32::from(b)),
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // let mut sphere_material = Material::default();

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random::vec3(0.0, 1.0) * random::vec3(0.0, 1.0);
                    let sphere_material = Material::lambertian(albedo);
                    let center2 = center + Direction::new(0.0, random::number(0.0, 0.5), 0.0);
                    world.add(HittableObject::moving_sphere(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random::vec3(0.5, 1.0);
                    let fuzz: f32 = random::number(0.0, 0.5);
                    let sphere_material = Material::metal(albedo, fuzz);
                    world.add(HittableObject::sphere(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Material::dielectric(1.5);
                    world.add(HittableObject::sphere(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::dielectric(1.5);
    world.add(HittableObject::sphere(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));

    let material2 = Material::lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(HittableObject::sphere(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));

    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(HittableObject::sphere(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));

    let world = HittableObject::bvh_node(world.objects());

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = Point::new(13.0, 2.0, 3.0);
    cam.look_at = Point::new(0.0, 0.0, 0.0);
    cam.vup = Direction::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

fn two_spheres() {
    let mut world = HittableList::default();

    let checker = Texture::checker(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world.add(HittableObject::sphere(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        Material::lambertian_with_texture(checker.clone()),
    ));
    world.add(HittableObject::sphere(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        Material::lambertian_with_texture(checker),
    ));

    let world = HittableObject::bvh_node(world.objects());

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400u16;
    cam.samples_per_pixel = 100u16;
    cam.max_depth = 50u16;

    cam.vfov = 20.0;
    cam.look_from = Point::new(13.0, 2.0, 3.0);
    cam.look_at = Point::new(0.0, 0.0, 0.0);
    cam.vup = Direction::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}
