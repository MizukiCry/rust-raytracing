use crate::utils::*;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub t: f64,
    pub material: Rc<Box<dyn Material>>,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        t: f64,
        material: Rc<Box<dyn Material>>,
        normal: Vec3,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            t,
            material,
            normal,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal).is_sign_negative();
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new(
            Vec3::default(),
            f64::default(),
            Rc::new(default_material()),
            Vec3::default(),
            bool::default(),
        )
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Rc<Box<dyn Hittable>>>,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<Box<dyn Hittable>>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<Box<dyn Hittable>>) {
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new(Vec::default())
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_t = ray_t.max;

        for object in &self.objects {
            let mut temp_record = HitRecord::default();
            if object.hit(ray, Interval::new(ray_t.min, closest_t), &mut temp_record) {
                hit_anything = true;
                closest_t = temp_record.t;
                *record = temp_record;
            }
        }

        hit_anything
    }
}
