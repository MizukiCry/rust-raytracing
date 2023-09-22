use crate::*;

#[derive(Default)]
pub struct SpherePDF {}

impl PDF for SpherePDF {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit()
    }
}
