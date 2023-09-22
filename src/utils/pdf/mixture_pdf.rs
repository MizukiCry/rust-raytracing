use crate::*;

pub struct MixturePDF {
    p0: Rc<dyn PDF>,
    p1: Rc<dyn PDF>,
}

impl MixturePDF {
    pub fn new(p0: Rc<dyn PDF>, p1: Rc<dyn PDF>) -> Self {
        Self { p0, p1 }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * (self.p0.value(direction) + self.p1.value(direction))
    }

    fn generate(&self) -> Vec3 {
        if random_f64() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
