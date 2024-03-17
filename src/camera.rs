use crate::color::{self, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Scatter;
use crate::ray::{Direction, Point, Ray};
use crate::utility;
use indicatif::ProgressBar;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64, // Vertical view angle
    pub look_from: Point,
    pub look_at: Point,
    pub vup: Direction,
    image_height: i32,
    center: Point,
    pixel00_location: Point,
    pixel_delta_u: Direction,
    pixel_delta_v: Direction,
    u: Direction,
    v: Direction, 
    w: Direction,
}
impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        // Render
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let pb = ProgressBar::new(self.image_height as u64);

        pb.println(format!("Starting"));
        for j in 0..self.image_height {
            pb.inc(1);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..(self.samples_per_pixel) {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                color::write_color(pixel_color, self.samples_per_pixel);
            }
        }
        pb.println(format!("Done"));
    }
    fn initialize(&mut self) {
        // Calculate image_height, ensure it is at least 1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.look_from;

        // Camera
        let focal_length = (self.look_from - self.look_at).length();
        let theta = self.vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        // self.center = Point::new(0.0, 0.0, 0.0);

        // Calculate u, v, w unit basis vectors for the camera coordinate frame.
        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = (self.vup.cross(self.w)).unit_vector();
        self.w = self.w.cross(self.u);

        // The vectors along the edges of the viewport
        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * (-viewport_height);

        // Horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.w*focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_location =
            viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_location
            + (self.pixel_delta_u * i as f64)
            + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(&ray_origin, &ray_direction);
    }

    fn pixel_sample_square(&self) -> Point {
        let px = -0.5 + utility::random_double(0.0, 1.0);
        let py = -0.5 + utility::random_double(0.0, 1.0);
        return self.pixel_delta_u * px + self.pixel_delta_v * py;
    }

    fn ray_color(&self, ray: &Ray, depth: i32, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec.mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return self.ray_color(&scattered, depth - 1, world) * attenuation;
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }
}
