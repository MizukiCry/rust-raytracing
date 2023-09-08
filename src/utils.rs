use crate::vec3::Vec3;

#[allow(dead_code)]
pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-8
}

#[allow(dead_code)]
pub fn equal_vec3(a: Vec3, b: Vec3) -> bool {
    equal(a.x, b.x) && equal(a.y, b.y) && equal(a.z, b.z)
}