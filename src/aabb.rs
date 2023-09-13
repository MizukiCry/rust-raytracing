mod interval;
pub use interval::*;

use crate::*;

/// Axis-aligned bounding box
#[derive(Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec3(a: &Vec3, b: &Vec3) -> Self {
        Self::new(
            Interval::new_adjusted(a.x, b.x),
            Interval::new_adjusted(a.y, b.y),
            Interval::new_adjusted(a.z, b.z),
        )
    }

    pub fn union(&self, x: &Aabb) -> Self {
        Self::new(self.x.union(&x.x), self.y.union(&x.y), self.z.union(&x.z))
    }

    pub fn unions(&mut self, x: &Aabb) {
        self.x.unions(&x.x);
        self.y.unions(&x.y);
        self.z.unions(&x.z);
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool {
        let mut inv = 1.0 / ray.direction.x;
        ray_t.intersects(&Interval::new_adjusted(
            (self.x.min - ray.origin.x) * inv,
            (self.x.max - ray.origin.x) * inv,
        ));
        if ray_t.empty() {
            return false;
        }

        inv = 1.0 / ray.direction.y;
        ray_t.intersects(&Interval::new_adjusted(
            (self.y.min - ray.origin.y) * inv,
            (self.y.max - ray.origin.y) * inv,
        ));
        if ray_t.empty() {
            return false;
        }

        inv = 1.0 / ray.direction.z;
        ray_t.intersects(&Interval::new_adjusted(
            (self.z.min - ray.origin.z) * inv,
            (self.z.max - ray.origin.z) * inv,
        ));
        !ray_t.empty()
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self::new(
            Interval::default(),
            Interval::default(),
            Interval::default(),
        )
    }
}
