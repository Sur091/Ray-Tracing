use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Scatter;
use crate::random;
use crate::ray::{Point, Ray};
use crate::texture::Texture;

#[derive(Debug, Clone, Default)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    pub const fn new(a: Color) -> Self {
        Self::new_with_texture(Texture::solid_color(a))
    }
    pub const fn new_with_texture(a: Texture) -> Self {
        Self { albedo: a }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random::unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.abs_diff_eq(Point::new(0.0, 0.0, 0.0), 1e-8) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
