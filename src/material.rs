pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

use crate::utils::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
