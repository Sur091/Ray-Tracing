use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::{Direction, Ray};
use crate::utility;

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Clone, Default)]
pub enum Material {
    #[default]
    NoMaterial,
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn lambertian(a: &Color) -> Self {
        Self::Lambertian(Lambertian::new(a))
    }
    pub fn metal(a: &Color, f: f64) -> Self {
        Self::Metal(Metal::new(a, f))
    }
    pub fn dielectric(index_of_refraction: f64) -> Self {
        Self::Dielectric(Dielectric::new(index_of_refraction))
    }
}

impl Scatter for Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Lambertian(lamb) => lamb.scatter(r_in, rec, attenuation, scattered),
            Self::Metal(met) => met.scatter(r_in, rec, attenuation, scattered),
            Self::Dielectric(die) => die.scatter(r_in, rec, attenuation, scattered),
            Self::NoMaterial => false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: *a }
    }
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Direction::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

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
            &rec.p,
            &(reflected + Direction::random_unit_vector() * self.fuzz),
        );
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ir: f64, // Index of refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
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
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = 1.0f64.min(-(rec.normal.dot(unit_direction)));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio)
                > utility::random_double(0.0, 1.0)
        {
            Direction::reflect(unit_direction, rec.normal)
        } else {
            Direction::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(&rec.p, &direction);

        return true;
    }
}
