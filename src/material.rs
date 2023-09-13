mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

use crate::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

// pub struct DefaultMaterial {}
// impl Material for DefaultMaterial {}
// pub static DEFAULT_MATERIAL: DefaultMaterial = DefaultMaterial {};
