use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::{random, utility};

#[derive(Debug, Clone, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(a: &Color, f: f32) -> Self {
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
        let reflected = utility::reflect(r_in.direction().normalize(), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + random::unit_vector() * self.fuzz,
            r_in.time(),
        );
        *attenuation = self.albedo;
        scattered.direction().dot(rec.normal) > 0.0
    }
}
