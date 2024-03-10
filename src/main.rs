use color::Color;
// use log::{info,warn};
use indicatif::ProgressBar;

mod color;
mod ray;
mod vec3;
mod hittable;
mod hittable_list;
mod sphere;

use vec3::Vec3;
use ray::{Point, Ray};

use crate::color::write_color;

fn main() {
    // Iamge
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400i32;

    // Calculate image_height, ensure it is at least 1
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // The vectors along the edges of the viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center 
        - Vec3::new(0.0, 0.0, focal_length) 
        - viewport_u / 2.0 
        - viewport_v / 2.0;
    let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    let pb = ProgressBar::new(image_height as u64);

    pb.println(format!("Starting"));
    for j in 0..image_height {
        pb.inc(1);
        for i in 0..image_width {
            let pixel_center = pixel00_location + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center; 
            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    pb.println(format!("Done"));
}

fn hit_shpere(center: Point, radius: f64, ray: &Ray) -> f64 {
        let oc = ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_shpere(Point::new(0.0, 0.0, -1.0), 0.5, r) ;
    if t > 0.0 {
        let normal = (r.at(t) - Point::new(0.0,0.0,-1.0)).unit_vector();
        return Color::new(normal.x()+1.0, normal.y()+1.0, normal.z()+1.0) * 0.5;
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0,1.0,1.0)*(1.0-a) + Color::new(0.5,0.7,1.0)*a
}
