use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;

mod dielectric;
mod lambertian;
mod metal;

pub trait Scatter {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
}

impl Default for Material {
    fn default() -> Self {
        Self::dielectric(1.5)
    }
}

impl Material {
    pub const fn lambertian(a: Color) -> Self {
        Self::Lambertian(lambertian::Lambertian::new(a))
    }
    pub const fn lambertian_with_texture(a: Texture) -> Self {
        Self::Lambertian(lambertian::Lambertian::new_with_texture(a))
    }
    pub fn metal(a: Color, f: f32) -> Self {
        Self::Metal(metal::Metal::new(&a, f))
    }
    pub const fn dielectric(index_of_refraction: f32) -> Self {
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
        }
    }
}
