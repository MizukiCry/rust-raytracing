use crate::*;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bounding_box: Aabb,
}

impl HittableList {
    fn new(objects: Vec<Rc<dyn Hittable>>, bounding_box: Aabb) -> Self {
        Self {
            objects,
            bounding_box,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bounding_box.unions(object.bounding_box());
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new(Vec::default(), Aabb::default())
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_t = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_t), record) {
                hit_anything = true;
                closest_t = record.t;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.objects[random_range_i32(0, self.objects.len() as i32 - 1) as usize].random(o)
    }
}
