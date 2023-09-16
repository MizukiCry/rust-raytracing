use crate::*;

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bounding_box: Aabb,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bounding_box = *object.bounding_box() + offset;
        Self {
            object,
            offset,
            bounding_box,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        if !self.object.hit(
            &Ray::new(ray.origin - self.offset, ray.direction, ray.time),
            ray_t,
            record,
        ) {
            return false;
        }
        record.p += self.offset;
        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
