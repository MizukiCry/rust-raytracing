use crate::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
    is_moving: bool,
    moving_direction: Vec3,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        let r3 = Vec3::new(radius, radius, radius);
        Self {
            center,
            radius,
            material,
            is_moving: false,
            moving_direction: Vec3::default(),
            bounding_box: Aabb::from_vec3(&(center - r3), &(center + r3)),
        }
    }

    pub fn new_moving(
        center1: Vec3,
        center2: Vec3,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> Self {
        let r3 = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_vec3(&(center1 - r3), &(center1 + r3));
        let box2 = Aabb::from_vec3(&(center2 - r3), &(center2 + r3));
        Self {
            center: center1,
            radius,
            material,
            is_moving: true,
            moving_direction: center2 - center1,
            bounding_box: box1.union(&box2),
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        if self.is_moving {
            self.center + time * self.moving_direction
        } else {
            self.center
        }
    }

    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        (
            (f64::atan2(-p.z, p.x) + PI) / (2.0 * PI),
            f64::acos(-p.y) / PI,
        )
    }

    fn random_to_sphere(radius: f64, dis_squared: f64) -> Vec3 {
        let r1 = random_f64();
        let r2 = random_f64();
        let z = 1.0 + r2 * ((1.0 - radius * radius / dis_squared).sqrt() - 1.0);
        let phi = 2.0 * PI * r1;
        Vec3::new(
            phi.cos() * (1.0 - z * z).sqrt(),
            phi.sin() * (1.0 - z * z).sqrt(),
            z,
        )
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(
            Vec3::default(),
            f64::default(),
            Rc::new(Lambertian::default()),
        )
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant.is_sign_negative() {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.p = ray.at(root);
        record.t = root;
        let outward_normal = (record.p - center) / self.radius;
        (record.u, record.v) = Sphere::get_sphere_uv(&outward_normal);
        record.material = Rc::clone(&self.material);
        record.set_face_normal(ray, outward_normal);

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let mut record = HitRecord::default();
        if !self.hit(
            &Ray::new(*o, *v, 0.0),
            Interval::new(0.0001, f64::INFINITY),
            &mut record,
        ) {
            return 0.0;
        }
        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - *o).length_squared()).sqrt();
        1.0 / (2.0 * PI * (1.0 - cos_theta_max))
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - *o;
        let base = ONB::from(&direction);
        base.local_vec3(&Self::random_to_sphere(
            self.radius,
            direction.length_squared(),
        ))
    }
}
