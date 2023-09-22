use crate::*;

pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self { u, v, w }
    }

    pub fn from(w: &Vec3) -> Self {
        let w = w.unit();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).unit();
        let u = w.cross(v);
        Self::new(u, v, w)
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u + b * self.v + c * self.w
    }

    pub fn local_vec3(&self, a: &Vec3) -> Vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }
}
