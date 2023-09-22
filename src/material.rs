mod lambertian;
pub use lambertian::*;

mod metal;
pub use metal::*;

mod dielectric;
pub use dielectric::*;

mod diffuse_light;
pub use diffuse_light::*;

mod isotropic;
pub use isotropic::*;

use crate::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool;

    fn scattering_pdf(&self, _ray: &Ray, _record: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _ray: &Ray, _record: &HitRecord, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}

// pub struct DefaultMaterial {}
// impl Material for DefaultMaterial {}
// pub static DEFAULT_MATERIAL: DefaultMaterial = DefaultMaterial {};
