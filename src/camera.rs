use crate::color::{self, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::{Direction, Point, Ray};
use indicatif::ProgressBar;
use crate::utility;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point,
    pixel00_location: Point,
    pixel_delta_u: Direction,
    pixel_delta_v: Direction,
}
impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
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
                let mut pixel_color = Color::new(0.0,0.0,0.0);
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

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.center = Point::new(0.0, 0.0, 0.0);

        // The vectors along the edges of the viewport
        let viewport_u = Point::new(viewport_width, 0.0, 0.0);
        let viewport_v = Point::new(0.0, -viewport_height, 0.0);

        // Horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Point::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_location =
            viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_location + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
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

    fn ray_color(&self, ray: &Ray, depth: i32, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = Direction::random_on_hemisphere(&rec.normal);
            return self.ray_color(&Ray::new(&rec.p, &direction), depth-1, world) *0.5;
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}