pub use crate::hittable::{HitRecord, Hittable, HittableList};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::{print_color, Vec3};
pub use crate::interval::Interval;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-8
}

pub fn equal_vec3(a: Vec3, b: Vec3) -> bool {
    equal(a.x, b.x) && equal(a.y, b.y) && equal(a.z, b.z)
}

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}
