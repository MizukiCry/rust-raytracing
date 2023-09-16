mod lambertian;
pub use lambertian::*;

mod metal;
pub use metal::*;

mod dielectric;
pub use dielectric::*;

mod diffuse_light;
pub use diffuse_light::*;

use crate::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}

// pub struct DefaultMaterial {}
// impl Material for DefaultMaterial {}
// pub static DEFAULT_MATERIAL: DefaultMaterial = DefaultMaterial {};
