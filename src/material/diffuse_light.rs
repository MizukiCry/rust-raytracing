use crate::*;

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn from_color(color: Vec3) -> Self {
        Self {
            emit: Rc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray: &Ray,
        _record: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.color(u, v, p)
    }
}
