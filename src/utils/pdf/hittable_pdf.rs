use crate::*;

pub struct HittablePDF {
    objects: Rc<dyn Hittable>,
    origin: Vec3,
}

impl HittablePDF {
    pub fn new(objects: Rc<dyn Hittable>, origin: Vec3) -> Self {
        Self { objects, origin }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}
