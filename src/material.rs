use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

mod lambertian;
mod metal;
mod dielectric;

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
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
}

impl Material {
    pub fn lambertian(a: Color) -> Self {
        Self::Lambertian(lambertian::Lambertian::new(&a))
    }
    pub fn metal(a: Color, f: f64) -> Self {
        Self::Metal(metal::Metal::new(&a, f))
    }
    pub fn dielectric(index_of_refraction: f64) -> Self {
        Self::Dielectric(dielectric::Dielectric::new(index_of_refraction))
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
