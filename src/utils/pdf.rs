use crate::*;

mod sphere_pdf;
pub use sphere_pdf::*;

mod cosine_pdf;
pub use cosine_pdf::*;

mod hittable_pdf;
pub use hittable_pdf::*;

mod mixture_pdf;
pub use mixture_pdf::*;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}
