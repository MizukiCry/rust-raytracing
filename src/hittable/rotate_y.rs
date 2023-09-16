use crate::*;

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Aabb,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degree_to_radian(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = *object.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
                    let (x, z) = (
                        cos_theta * x + sin_theta * z,
                        -sin_theta * x + cos_theta * z,
                    );

                    if min.x > x {
                        min.x = x;
                    }
                    if min.y > y {
                        min.y = y;
                    }
                    if min.z > z {
                        min.z = z;
                    }
                    if max.x > x {
                        max.x = x;
                    }
                    if max.y > y {
                        max.y = y;
                    }
                    if max.z > z {
                        max.z = z;
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box: Aabb::from_vec3(&min, &max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let origin = Vec3::new(
            self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
            ray.origin.y,
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
        );
        let direction = Vec3::new(
            self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
            ray.direction.y,
            self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z,
        );

        if !self
            .object
            .hit(&Ray::new(origin, direction, ray.time), ray_t, record)
        {
            return false;
        }

        record.p = Vec3::new(
            self.cos_theta * record.p.x + self.sin_theta * record.p.z,
            record.p.y,
            -self.sin_theta * record.p.x + self.cos_theta * record.p.z,
        );
        record.normal = Vec3::new(
            self.cos_theta * record.normal.x + self.sin_theta * record.normal.z,
            record.normal.y,
            -self.sin_theta * record.normal.x + self.cos_theta * record.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
