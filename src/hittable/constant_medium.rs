use crate::*;

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, texture: Rc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase: Rc::new(Isotropic::new(texture)),
        }
    }

    pub fn from_color(boundary: Rc<dyn Hittable>, density: f64, color: Vec3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase: Rc::new(Isotropic::from_color(color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        if !self
            .boundary
            .hit(ray, Interval::new(0.0, ray_t.max), &mut temp_record)
        {
            return false;
        }
        let t1 = temp_record.t.max(ray_t.min);
        if !self
            .boundary
            .hit(ray, Interval::new(t1 + 0.0001, ray_t.max), &mut temp_record)
        {
            return false;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (temp_record.t - t1) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64().ln();
        if hit_distance > distance_inside_boundary {
            return false;
        }

        record.t = t1 + hit_distance / ray_length;
        record.p = ray.at(record.t);
        record.normal = Vec3::new(1.0, 0.0, 0.0);
        record.front_face = true;
        record.material = Rc::clone(&self.phase);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
