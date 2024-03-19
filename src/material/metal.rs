use crate::material::Scatter;
use crate::hittable::HitRecord;
use crate::ray::{Direction, Ray};
use crate::color::Color;

#[derive(Debug, Clone, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: &Color, f: f64) -> Self {
        Self {
            albedo: *a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Direction::reflect(Direction::unit_vector(&r_in.direction()), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Direction::random_unit_vector() * self.fuzz,
            r_in.time(),
        );
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}
