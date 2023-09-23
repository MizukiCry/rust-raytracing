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

#[derive(Default)]
pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub pdf: Option<Rc<dyn PDF>>,
    pub skip_pdf_ray: Ray,
}

pub trait Material {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord, _srecord: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _ray: &Ray, _record: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emitted(&self, _ray: &Ray, _record: &HitRecord, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}

#[derive(Default)]
pub struct DefaultMaterial {}
impl Material for DefaultMaterial {}
