use crate::*;

pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Default for SolidColor {
    fn default() -> Self {
        Self::new(Vec3::default())
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}
