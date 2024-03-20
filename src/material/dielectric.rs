use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Scatter;
use crate::ray::Ray;
use crate::{random, utility};

#[derive(Debug, Clone)]
pub struct Dielectric {
    ir: f32, // Index of refraction
}

impl Dielectric {
    pub const fn new(index_of_refraction: f32) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        // r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        (1.0 - r0).mul_add((1.0 - cosine).powi(5), r0)
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f32 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = 1.0f32.min(-(rec.normal.dot(unit_direction)));
        // let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let sin_theta = (cos_theta.mul_add(-cos_theta, 1.0)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > random::number(0.0, 1.0)
        {
            utility::reflect(unit_direction, rec.normal)
        } else {
            utility::refract(unit_direction, rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction, r_in.time());

        true
    }
}
