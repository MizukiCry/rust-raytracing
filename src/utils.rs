pub use crate::camera::Camera;
pub use crate::hittable::{HitRecord, Hittable, HittableList};
pub use crate::material::{Dielectric, Lambertian, Material, Metal};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::Vec3;

pub use std::rc::Rc;

const EPS: f64 = 1e-8;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

pub fn equal_vec3(a: Vec3, b: Vec3) -> bool {
    equal(a.x, b.x) && equal(a.y, b.y) && equal(a.z, b.z)
}

pub fn is_zero_vec3(x: Vec3) -> bool {
    equal_vec3(x, Vec3::default())
}

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}

pub fn random_u64() -> u64 {
    static mut X: u64 = 721;
    unsafe {
        X ^= X >> 12;
        X ^= X << 25;
        X ^= X >> 27;
        X.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

/// Returns a random `f64` number in [0, 1]
pub fn random_f64() -> f64 {
    random_u64() as f64 / u64::MAX as f64
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
