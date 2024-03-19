mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;


use camera::Camera;
use color::Color;
use ray::{Direction, Point};
use hittable::HittableObject;
use hittable_list::HittableList;
use material::Material;

fn main() {
    // World
    let mut world = HittableList::default();

    let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(HittableObject::sphere(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utility::random_double(0.0, 1.0);
            let center = Point::new(
                a as f64 + 0.9*utility::random_double(0.0, 1.0),
                0.2, 
                b as f64 + 0.9*utility::random_double(0.0, 1.0)
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // let mut sphere_material = Material::default();

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let sphere_material = Material::lambertian(albedo);
                    let center2 = center + Direction::new(0.0, utility::random_double(0.0, 0.5), 0.0);
                    world.add(HittableObject::moving_sphere(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95{
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = utility::random_double(0.0, 0.5);
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
    world.add(HittableObject::sphere(Point::new(0.0,1.0,0.0), 1.0, material1));
    
    let material2 = Material::lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(HittableObject::sphere(Point::new(-4.0,1.0,0.0), 1.0, material2));
    
    let material3 = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(HittableObject::sphere(Point::new(4.0,1.0,0.0), 1.0, material3));


    // let r: f64 = f64::cos(std::f64::consts::FRAC_PI_4);

    // let material_groud = Material::lambertian(Color::new(0.8, 0.8, 0.0));
    // let material_center = Material::lambertian(Color::new(0.1, 0.2, 0.5));
    // let material_left = Material::dielectric(1.5);
    // let material_left_c = Material::dielectric(1.5);
    // let material_right = Material::metal(Color::new(0.8,0.6,0.2), 0.0);

    // world.add(HittableObject::sphere(Point::new( 0.0, -100.5, -1.0),100.0,material_groud));
    // world.add(HittableObject::sphere(Point::new( 0.0,    0.0, -1.0),  0.5,material_center));
    // world.add(HittableObject::sphere(Point::new(-1.0,    0.0, -1.0),  0.5,material_left_c));
    // world.add(HittableObject::sphere(Point::new(-1.0,    0.0, -1.0), -0.4,material_left));
    // world.add(HittableObject::sphere(Point::new( 1.0,    0.0, -1.0),  0.5,material_right));

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
